use agent_orchestrator::{
    digest,
    gate::{evidence_path, snapshot, validate_report},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Repository(PathBuf);
impl Repository {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "orchestrator-gate-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        fs::create_dir(path.join("hooks")).unwrap();
        let fixture = Self(path);
        fixture.git(&["init", "-q"]);
        fixture.git(&["config", "user.name", "Gate Fixture"]);
        fixture.git(&["config", "user.email", "fixture@example.invalid"]);
        fs::write(fixture.0.join("source.txt"), "real source\n").unwrap();
        fixture.git(&["add", "source.txt"]);
        fixture.git(&["commit", "-qm", "Fixture"]);
        fixture
    }
    fn git(&self, args: &[&str]) -> String {
        let output = Command::new("git")
            .args([
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.fsmonitor=false",
                "-c",
            ])
            .arg(format!("core.hooksPath={}", self.0.join("hooks").display()))
            .arg("-C")
            .arg(&self.0)
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success(), "Git fixture command failed");
        String::from_utf8(output.stdout).unwrap()
    }
    fn report(&self, current: &Value) -> Value {
        fs::create_dir_all(self.0.join("docs/quality")).unwrap();
        let evidence = self.0.join("docs/quality/check.txt");
        fs::write(&evidence, "actual fixture acceptance evidence").unwrap();
        let hash = digest(&evidence).unwrap();
        let checks: Vec<_> = [
            "build",
            "e2e",
            "code-review",
            "security-review",
            "evidence-review",
            "docs-review",
        ]
        .iter()
        .map(|kind| {
            json!({"id": kind, "kind": kind, "required": true,
                "status": "passed", "source_digest": current["source_digest"],
                "evidence": "docs/quality/check.txt", "evidence_sha256": hash})
        })
        .collect();
        json!({"format_version": 1, "source_digest": current["source_digest"],
            "blocking_findings": [], "unresolved_decisions": [], "checks": checks,
            "ci": {"head_sha": current["head_sha"], "checks": [
                {"id": "final-gate", "required": true, "status": "success"}]}})
    }
}
impl Drop for Repository {
    fn drop(&mut self) {
        // Only the generated fixture path, constructed directly under the temp root.
        if self.0.parent() == Some(std::env::temp_dir().as_path())
            && self
                .0
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("orchestrator-gate-")
        {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
}

#[test]
fn actual_git_source_and_evidence_freshness() {
    let repo = Repository::new();
    let initial = snapshot(&repo.0).unwrap();
    let report = repo.report(&initial);
    assert_eq!(
        initial,
        snapshot(&repo.0).unwrap(),
        "Evidence must not invalidate source"
    );
    for stage in ["pre-push", "merge", "deploy"] {
        let result = validate_report(&repo.0, &report, stage, &initial).unwrap();
        assert_eq!(result["record_validation"], "passed");
        for flag in [
            "test_execution_verified",
            "remote_ci_verified",
            "external_action_authorized",
        ] {
            assert_eq!(result[flag], false);
        }
    }
    fs::write(repo.0.join("source.txt"), "changed source\n").unwrap();
    let changed = snapshot(&repo.0).unwrap();
    assert_ne!(initial["source_digest"], changed["source_digest"]);
    assert!(validate_report(&repo.0, &report, "pre-push", &changed).is_err());
}

#[test]
fn untracked_ignored_and_deleted_paths_are_handled() {
    let repo = Repository::new();
    fs::write(repo.0.join(".gitignore"), "ignored.txt\n").unwrap();
    let initial = snapshot(&repo.0).unwrap();
    fs::write(repo.0.join("ignored.txt"), "ignored").unwrap();
    assert_eq!(initial, snapshot(&repo.0).unwrap());
    fs::write(repo.0.join("untracked.txt"), "new source").unwrap();
    let untracked = snapshot(&repo.0).unwrap();
    assert_ne!(initial["source_digest"], untracked["source_digest"]);
    fs::remove_file(repo.0.join("source.txt")).unwrap();
    assert_ne!(
        untracked["source_digest"],
        snapshot(&repo.0).unwrap()["source_digest"]
    );
}

#[test]
fn exact_repository_root_and_real_head_required() {
    let repo = Repository::new();
    fs::create_dir(repo.0.join("nested")).unwrap();
    assert!(snapshot(&repo.0.join("nested")).is_err());
    let empty = repo.0.join("empty");
    fs::create_dir(&empty).unwrap();
    assert!(snapshot(&empty).is_err());
}

#[test]
fn uninitialized_submodule_is_blocked() {
    let repo = Repository::new();
    let head = repo.git(&["rev-parse", "HEAD"]);
    let spec = format!("160000,{},submodule", head.trim());
    repo.git(&["update-index", "--add", "--cacheinfo", &spec]);
    assert!(snapshot(&repo.0).is_err());
}

#[test]
fn evidence_containment_and_hash_are_required() {
    let repo = Repository::new();
    let current = snapshot(&repo.0).unwrap();
    let mut report = repo.report(&current);
    for invalid in [
        "",
        "source.txt",
        "docs/quality",
        "docs/quality-extra/check.txt",
        "docs/quality/../../source.txt",
    ] {
        assert!(evidence_path(&repo.0, invalid).is_err());
    }
    assert!(
        evidence_path(
            &repo.0,
            repo.0.join("docs/quality/check.txt").to_str().unwrap()
        )
        .is_err()
    );
    report["checks"][0]["evidence_sha256"] = json!("0".repeat(64));
    assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    report = repo.report(&current);
    fs::write(repo.0.join("docs/quality/check.txt"), "tampered").unwrap();
    assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
}

#[test]
fn required_record_negative_paths_block() {
    let repo = Repository::new();
    let current = snapshot(&repo.0).unwrap();
    let original = repo.report(&current);
    for format in [json!(true), json!("1"), json!(1.0), json!(0), json!(null)] {
        let mut report = original.clone();
        report["format_version"] = format;
        assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    }
    for field in ["blocking_findings", "unresolved_decisions"] {
        let mut report = original.clone();
        report.as_object_mut().unwrap().remove(field);
        assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
        report[field] = json!(["unresolved"]);
        assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    }
    for status in ["failed", "skipped", "neutral", "pending", "success"] {
        let mut report = original.clone();
        report["checks"][0]["status"] = json!(status);
        assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    }
    for field in [
        "id",
        "kind",
        "required",
        "evidence",
        "evidence_sha256",
        "source_digest",
    ] {
        let mut report = original.clone();
        report["checks"][0].as_object_mut().unwrap().remove(field);
        assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    }
    let mut report = original.clone();
    report["checks"][0]["id"] = report["checks"][1]["id"].clone();
    assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    report = original.clone();
    report["checks"][0]["required"] = json!(false);
    assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    report = original.clone();
    report["checks"][0]["required"] = json!(1);
    assert!(validate_report(&repo.0, &report, "pre-push", &current).is_err());
    assert!(validate_report(&repo.0, &original, "unknown", &current).is_err());
}

#[test]
fn ci_requires_actual_head_unique_explicit_success_and_final_gate() {
    let repo = Repository::new();
    let current = snapshot(&repo.0).unwrap();
    let original = repo.report(&current);
    for status in [
        "skipped",
        "neutral",
        "pending",
        "failure",
        "passed",
        "cancelled",
    ] {
        let mut report = original.clone();
        report["ci"]["checks"][0]["status"] = json!(status);
        assert!(validate_report(&repo.0, &report, "merge", &current).is_err());
    }
    for field in ["id", "required", "status"] {
        let mut report = original.clone();
        report["ci"]["checks"][0]
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert!(validate_report(&repo.0, &report, "deploy", &current).is_err());
    }
    let mut report = original.clone();
    report["ci"]["head_sha"] = json!("stale");
    assert!(validate_report(&repo.0, &report, "merge", &current).is_err());
    report = original.clone();
    report["ci"]["checks"][0]["required"] = json!(false);
    assert!(validate_report(&repo.0, &report, "merge", &current).is_err());
    report = original.clone();
    report["ci"]["checks"][0]["required"] = json!("true");
    assert!(validate_report(&repo.0, &report, "merge", &current).is_err());
    report = original.clone();
    let duplicate = report["ci"]["checks"][0].clone();
    report["ci"]["checks"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert!(validate_report(&repo.0, &report, "merge", &current).is_err());
    report = original.clone();
    report["ci"]["checks"][0]["id"] = json!("other-check");
    assert!(validate_report(&repo.0, &report, "merge", &current).is_err());
    report = original.clone();
    report.as_object_mut().unwrap().remove("ci");
    assert!(validate_report(&repo.0, &report, "pre-push", &current).is_ok());
    assert!(validate_report(&repo.0, &report, "merge", &current).is_err());
}

#[cfg(unix)]
#[test]
fn links_hash_their_text_and_evidence_cannot_escape() {
    use std::os::unix::fs::symlink;
    let repo = Repository::new();
    symlink("source.txt", repo.0.join("source-link")).unwrap();
    let initial = snapshot(&repo.0).unwrap();
    fs::remove_file(repo.0.join("source-link")).unwrap();
    symlink("different-target", repo.0.join("source-link")).unwrap();
    assert_ne!(
        initial["source_digest"],
        snapshot(&repo.0).unwrap()["source_digest"]
    );
    fs::create_dir_all(repo.0.join("docs/quality")).unwrap();
    symlink("../../source.txt", repo.0.join("docs/quality/escaped")).unwrap();
    assert!(evidence_path(&repo.0, "docs/quality/escaped").is_err());
}

#[cfg(unix)]
#[test]
fn execution_mode_changes_fingerprint() {
    use std::os::unix::fs::PermissionsExt;
    let repo = Repository::new();
    let initial = snapshot(&repo.0).unwrap();
    fs::set_permissions(repo.0.join("source.txt"), fs::Permissions::from_mode(0o755)).unwrap();
    assert_ne!(
        initial["source_digest"],
        snapshot(&repo.0).unwrap()["source_digest"]
    );
}
