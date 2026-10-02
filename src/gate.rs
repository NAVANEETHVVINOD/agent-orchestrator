//! Read-only release-record validation. A valid record does not attest execution.
use crate::{Result, digest};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeSet, HashSet},
    fs,
    io::Read,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const MAX_FILES: usize = 30_000;
const MAX_GIT_BYTES: u64 = 20 * 1024 * 1024;
const REQUIRED_KINDS: [&str; 6] = [
    "build",
    "e2e",
    "code-review",
    "security-review",
    "evidence-review",
    "docs-review",
];

fn git(root: &Path, args: &[&str]) -> Result<String> {
    let mut child = Command::new("git")
        .args(["-c", "core.fsmonitor=false", "-C"])
        .arg(root)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "Read-only Git inspection unavailable")?;
    let stdout = child.stdout.take().ok_or("Git output unavailable")?;
    let stderr = child.stderr.take().ok_or("Git output unavailable")?;
    let (sender, receiver) = mpsc::channel();
    fn capture(
        reader: impl Read + Send + 'static,
        output: bool,
        sender: mpsc::Sender<(bool, Result<Vec<u8>>)>,
    ) {
        thread::spawn(move || {
            let mut bytes = Vec::new();
            let result = reader
                .take(MAX_GIT_BYTES + 1)
                .read_to_end(&mut bytes)
                .map_err(|_| "Git output unavailable".to_owned())
                .and_then(|_| {
                    if bytes.len() as u64 > MAX_GIT_BYTES {
                        Err("Git listing exceeds the supported size".into())
                    } else {
                        Ok(bytes)
                    }
                });
            let _ = sender.send((output, result));
        });
    }
    capture(stdout, true, sender.clone());
    capture(stderr, false, sender);
    let started = Instant::now();
    let mut output = None;
    let mut received = 0;
    let mut status = None;
    let inspection = loop {
        while let Ok((is_output, result)) = receiver.try_recv() {
            received += 1;
            match result {
                Ok(bytes) => {
                    if is_output {
                        output = Some(bytes);
                    }
                }
                Err(error) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(error);
                }
            }
        }
        if status.is_none() {
            match child.try_wait() {
                Ok(value) => status = value,
                Err(_) => break Err("Read-only Git inspection unavailable".into()),
            }
        }
        if let Some(exit_status) = status {
            if !exit_status.success() {
                break Err("Read-only Git inspection failed; repository needs a real HEAD".into());
            }
            if received == 2 {
                break String::from_utf8(output.unwrap_or_default())
                    .map_err(|_| "Non-UTF-8 Git paths are unsupported".into());
            }
        }
        if started.elapsed() >= Duration::from_secs(20) {
            break Err("Read-only Git inspection timed out".into());
        }
        thread::sleep(Duration::from_millis(10));
    };
    if inspection.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    inspection
}

fn relative(name: &str) -> Result<PathBuf> {
    if name.is_empty() || name.contains('\0') {
        return Err("Invalid relative path".into());
    }
    let path = PathBuf::from(name);
    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
    {
        return Err("Invalid relative path".into());
    }
    Ok(path)
}

#[cfg(unix)]
fn executable(path: &Path) -> Result<bool> {
    use std::os::unix::fs::PermissionsExt;
    Ok(path
        .metadata()
        .map_err(|_| "Source unavailable")?
        .permissions()
        .mode()
        & 0o111
        != 0)
}

#[cfg(not(unix))]
fn executable(path: &Path) -> Result<bool> {
    Ok(path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            ["exe", "com", "bat", "cmd"]
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        }))
}

/// Fingerprint tracked and nonignored untracked source, excluding evidence files.
pub fn snapshot(project_root: &Path) -> Result<Value> {
    let root = fs::canonicalize(project_root).map_err(|_| "Project root unavailable")?;
    let actual = fs::canonicalize(git(&root, &["rev-parse", "--show-toplevel"])?.trim())
        .map_err(|_| "Git root unavailable")?;
    if root != actual {
        return Err("Choose the exact Git repository root".into());
    }
    let head = git(&root, &["rev-parse", "HEAD"])?.trim().to_owned();
    // An uninitialized submodule may have no directory; identify gitlinks by mode.
    if git(&root, &["ls-files", "--stage", "-z"])?
        .split('\0')
        .any(|entry| entry.starts_with("160000 "))
    {
        return Err("Submodules require separate source-freshness verification".into());
    }
    let listing = git(
        &root,
        &[
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ],
    )?;
    let names: BTreeSet<_> = listing
        .split('\0')
        .filter(|name| !name.is_empty())
        .collect();
    if names.len() > MAX_FILES {
        return Err("Repository exceeds the supported file count".into());
    }
    let mut hash = Sha256::new();
    let mut included = 0;
    for name in names {
        let rel = relative(name)?;
        if rel.starts_with("docs/quality") && rel.components().count() > 2 {
            continue;
        }
        let path = root.join(&rel);
        // Canonicalize the parent, not the final component: symlink text is source.
        let parent = fs::canonicalize(path.parent().ok_or("Invalid source path")?)
            .map_err(|_| "Source directory unavailable")?;
        if !parent.starts_with(&root) {
            return Err("Source directory resolves outside the selected project".into());
        }
        hash.update(name.as_bytes());
        hash.update(b"\0");
        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                let target = fs::read_link(&path).map_err(|_| "Source link unavailable")?;
                let target = target
                    .to_str()
                    .ok_or("Non-UTF-8 source links are unsupported")?;
                hash.update(b"link\0");
                hash.update(target.as_bytes());
            }
            Ok(metadata) if metadata.is_file() => {
                hash.update(b"file\0");
                hash.update(if executable(&path)? {
                    b"True\0".as_slice()
                } else {
                    b"False\0".as_slice()
                });
                hash.update(digest(&path)?.as_bytes());
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => hash.update(b"deleted"),
            Err(_) => return Err("Source unavailable".into()),
            Ok(_) => {
                return Err(
                    "Submodules/directories require separate source-freshness verification".into(),
                );
            }
        }
        hash.update(b"\0");
        included += 1;
    }
    Ok(
        json!({"head_sha": head, "source_digest": crate::hex(&hash.finalize()),
        "source_file_count": included, "excluded_evidence_directory": "docs/quality/"}),
    )
}

/// Resolve a regular evidence file strictly contained in this project's quality directory.
pub fn evidence_path(project_root: &Path, name: &str) -> Result<PathBuf> {
    let root = fs::canonicalize(project_root).map_err(|_| "Project root unavailable")?;
    let rel = relative(name)?;
    if !rel.starts_with("docs/quality") || rel == Path::new("docs/quality") {
        return Err("Evidence must stay under docs/quality/".into());
    }
    let quality = fs::canonicalize(root.join("docs/quality"))
        .map_err(|_| "Evidence directory unavailable")?;
    let path = fs::canonicalize(root.join(rel)).map_err(|_| "Evidence unavailable")?;
    if !quality.starts_with(&root) || !path.starts_with(&quality) || !path.is_file() {
        return Err("Evidence resolves outside its permitted directory".into());
    }
    Ok(path)
}

/// Check release records and hashes; never execute tests, inspect remote CI, or authorize actions.
pub fn validate_report(root: &Path, report: &Value, stage: &str, current: &Value) -> Result<Value> {
    if !["pre-push", "merge", "deploy"].contains(&stage) {
        return Err("Unsupported release stage".into());
    }
    if report.get("format_version").and_then(Value::as_u64) != Some(1) {
        return Err("Unsupported or absent report format".into());
    }
    let source = current
        .get("source_digest")
        .and_then(Value::as_str)
        .ok_or("Source snapshot missing")?;
    if report.get("source_digest").and_then(Value::as_str) != Some(source) {
        return Err("Report is stale for the current source".into());
    }
    for field in ["blocking_findings", "unresolved_decisions"] {
        if !report
            .get(field)
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty)
        {
            return Err(
                "Blocking findings and unresolved decisions must explicitly be empty lists".into(),
            );
        }
    }
    let checks = report
        .get("checks")
        .and_then(Value::as_array)
        .filter(|checks| !checks.is_empty())
        .ok_or("Actual required check records are missing")?;
    let mut ids = HashSet::new();
    let mut kinds = HashSet::new();
    for check in checks {
        let id = check
            .get("id")
            .and_then(Value::as_str)
            .filter(|id| !id.is_empty())
            .ok_or("Check identities must be nonempty and unique")?;
        if !ids.insert(id) {
            return Err("Check identities must be nonempty and unique".into());
        }
        let required = check
            .get("required")
            .and_then(Value::as_bool)
            .ok_or("Each check must explicitly state whether it is required")?;
        if required {
            if check.get("status").and_then(Value::as_str) != Some("passed") {
                return Err("A required check did not pass".into());
            }
            if check.get("source_digest").and_then(Value::as_str) != Some(source) {
                return Err("A required check is stale".into());
            }
            let path = evidence_path(
                root,
                check
                    .get("evidence")
                    .and_then(Value::as_str)
                    .ok_or("Required evidence path missing")?,
            )?;
            if check.get("evidence_sha256").and_then(Value::as_str) != Some(digest(&path)?.as_str())
            {
                return Err("Required evidence hash mismatch".into());
            }
            kinds.insert(
                check
                    .get("kind")
                    .and_then(Value::as_str)
                    .ok_or("Required check kind must be a string")?,
            );
        }
    }
    if REQUIRED_KINDS.iter().any(|kind| !kinds.contains(kind)) {
        return Err("Missing required check kinds".into());
    }
    if ["merge", "deploy"].contains(&stage) {
        let head = current
            .get("head_sha")
            .and_then(Value::as_str)
            .ok_or("Source HEAD missing")?;
        let ci = report.get("ci").ok_or("Required CI records are missing")?;
        if ci.get("head_sha").and_then(Value::as_str) != Some(head) {
            return Err("Reported CI must match the actual current Git HEAD".into());
        }
        let checks = ci
            .get("checks")
            .and_then(Value::as_array)
            .filter(|checks| !checks.is_empty())
            .ok_or("Required CI records are missing")?;
        let mut ids = HashSet::new();
        let mut final_gate = false;
        for check in checks {
            let id = check
                .get("id")
                .and_then(Value::as_str)
                .filter(|id| !id.is_empty())
                .ok_or("CI check identities must be nonempty and unique")?;
            if !ids.insert(id) {
                return Err("CI check identities must be nonempty and unique".into());
            }
            let required = check
                .get("required")
                .and_then(Value::as_bool)
                .ok_or("Each CI check must explicitly state whether it is required")?;
            if required {
                if id == "final-gate" {
                    final_gate = true;
                }
                if check.get("status").and_then(Value::as_str) != Some("success") {
                    return Err(
                        "Required CI checks must be success, not skipped/neutral/pending/failure"
                            .into(),
                    );
                }
            }
        }
        if !final_gate {
            return Err("Required final CI gate record is missing".into());
        }
    }
    let mut output = current
        .as_object()
        .ok_or("Source snapshot must be an object")?
        .clone();
    output.extend(
        json!({"record_validation": "passed", "stage": stage,
        "test_execution_verified": false, "remote_ci_verified": false,
        "external_action_authorized": false})
        .as_object()
        .expect("Object literal")
        .clone(),
    );
    Ok(Value::Object(output))
}
