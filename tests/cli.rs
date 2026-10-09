use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "orchestrator-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn path(&self) -> &str {
        self.0.to_str().unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_orchestrator"))
        .args(args)
        .output()
        .unwrap()
}
fn success(output: Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
fn blocked(output: Output) {
    assert!(!output.status.success());
    let v: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(v["external_action_authorized"], false);
}
fn copy(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for item in fs::read_dir(source).unwrap() {
        let item = item.unwrap();
        let name = item.file_name();
        if ["target", ".git", "__pycache__"].iter().any(|n| name == *n) {
            continue;
        }
        let dst = target.join(name);
        if item.file_type().unwrap().is_dir() {
            copy(&item.path(), &dst);
        } else {
            fs::copy(item.path(), dst).unwrap();
        }
    }
}
fn git(root: &Path, args: &[&str]) {
    let hooks = root.join(".fixture-hooks");
    fs::create_dir_all(&hooks).unwrap();
    let o = Command::new("git")
        .args([
            "-c",
            "core.fsmonitor=false",
            "-c",
            "commit.gpgsign=false",
            "-c",
        ])
        .arg(format!("core.hooksPath={}", hooks.display()))
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
}
#[test]
fn package_real_consumer_and_crlf_validation() {
    let f = Fixture::new();
    copy(Path::new(env!("CARGO_MANIFEST_DIR")), &f.0);
    let result = success(run(&["validate-package", "--root", f.path()]));
    assert!(result["skills"].as_u64().unwrap() >= 14);
    let skill = f.0.join("skills/global-orchestrator/SKILL.md");
    let text = fs::read_to_string(&skill).unwrap();
    fs::write(&skill, text.replace("\r\n", "\n").replace('\n', "\r\n")).unwrap();
    success(run(&["validate-package", "--root", f.path()]));
    fs::write(f.0.join("auth.json"), "{}").unwrap();
    blocked(run(&["validate-package", "--root", f.path()]));
}
#[test]
fn missing_markdown_target_blocks() {
    let f = Fixture::new();
    copy(Path::new(env!("CARGO_MANIFEST_DIR")), &f.0);
    fs::remove_file(f.0.join("docs/RUST.md")).unwrap();
    blocked(run(&["validate-package", "--root", f.path()]));
}
#[test]
fn route_cli_exact_boundaries_and_untrusted_inputs() {
    let f = Fixture::new();
    let input = f.0.join("route.json");
    let path = input.to_str().unwrap();
    let record = json!({"format_version":1,"available_routes":["qa","research"],"minimum_score":0.6,"minimum_margin":0.2,"proposals":[{"route":"qa","score":0.7},{"route":"research","score":0.5}]});
    fs::write(&input, record.to_string()).unwrap();
    let r = success(run(&["route-proposal", "--input", path]));
    assert_eq!(r["route"], "qa");
    assert_eq!(r["execution_authorized"], false);
    for invalid in [
        r#"{"unused":NaN}"#,
        r#"{"format_version":1,"format_version":1}"#,
        r#"{"score":{"$serde_json::private::Number":"0.9"}}"#,
    ] {
        fs::write(&input, invalid).unwrap();
        blocked(run(&["route-proposal", "--input", path]));
    }
    fs::write(&input, vec![b' '; 1024 * 1024 + 1]).unwrap();
    blocked(run(&["route-proposal", "--input", path]));
}

#[test]
fn workflow_cli_validates_bundled_roles_and_rejects_untrusted_records() {
    let f = Fixture::new();
    let input = f.0.join("workflow.json");
    let path = input.to_str().unwrap();
    fs::write(&input, include_str!("../examples/project-workflow.json")).unwrap();
    let result = success(run(&["validate-workflow", "--input", path]));
    assert_eq!(result["record_validation"], "passed");
    assert_eq!(result["role_count"], 14);
    assert_eq!(result["tools_installed"], false);
    assert_eq!(result["permissions_granted"], false);

    let sample = include_str!("../examples/project-workflow.json")
        .replace("\"format_version\": 1,", "\"format_version\": 1.0,");
    fs::write(&input, sample).unwrap();
    success(run(&["validate-workflow", "--input", path]));

    let mut unicode_name: Value =
        serde_json::from_str(include_str!("../examples/project-workflow.json")).unwrap();
    unicode_name["project"]["name"] = json!("界".repeat(120));
    fs::write(&input, unicode_name.to_string()).unwrap();
    success(run(&["validate-workflow", "--input", path]));
    unicode_name["project"]["name"] = json!("界".repeat(121));
    fs::write(&input, unicode_name.to_string()).unwrap();
    blocked(run(&["validate-workflow", "--input", path]));

    let mut padded_name: Value =
        serde_json::from_str(include_str!("../examples/project-workflow.json")).unwrap();
    padded_name["project"]["name"] = json!(" padded ");
    fs::write(&input, padded_name.to_string()).unwrap();
    blocked(run(&["validate-workflow", "--input", path]));

    let mut ecma_whitespace_name: Value =
        serde_json::from_str(include_str!("../examples/project-workflow.json")).unwrap();
    ecma_whitespace_name["project"]["name"] = json!("\u{0085}name\u{0085}");
    fs::write(&input, ecma_whitespace_name.to_string()).unwrap();
    success(run(&["validate-workflow", "--input", path]));

    for invalid in [
        r#"{"format_version":1,"format_version":1}"#,
        r#"{"format_version":1,"project":{},"roles":[],"required_quality_gates":[],"run":"whoami"}"#,
    ] {
        fs::write(&input, invalid).unwrap();
        blocked(run(&["validate-workflow", "--input", path]));
    }
    blocked(run(&[
        "validate-workflow",
        "--input",
        path,
        "--extra",
        "yes",
    ]));
}
#[test]
fn project_cli_real_git_report_and_source_change() {
    let f = Fixture::new();
    git(&f.0, &["init"]);
    fs::write(f.0.join("source.txt"), "real source").unwrap();
    git(&f.0, &["add", "source.txt"]);
    git(
        &f.0,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "-m",
            "fixture",
        ],
    );
    let snapshot = success(run(&[
        "project-gate",
        "--project-root",
        f.path(),
        "--snapshot",
    ]));
    let dir = f.0.join("docs/quality");
    fs::create_dir_all(&dir).unwrap();
    let evidence = dir.join("actual.txt");
    fs::write(
        &evidence,
        "Synthetic gate record used to test actual CLI validation, not an application test claim.",
    )
    .unwrap();
    let hash = agent_orchestrator::digest(&evidence).unwrap();
    let checks:Vec<_>=["build","e2e","code-review","security-review","evidence-review","docs-review"].iter().map(|kind|json!({"id":kind,"kind":kind,"required":true,"status":"passed","source_digest":snapshot["source_digest"],"evidence":"docs/quality/actual.txt","evidence_sha256":hash})).collect();
    let report = json!({"format_version":1,"source_digest":snapshot["source_digest"],"checks":checks,"blocking_findings":[],"unresolved_decisions":[]});
    fs::write(dir.join("report.json"), report.to_string()).unwrap();
    let args = [
        "project-gate",
        "--project-root",
        f.path(),
        "--report",
        "docs/quality/report.json",
        "--stage",
        "pre-push",
    ];
    let r = success(run(&args));
    assert_eq!(r["record_validation"], "passed");
    for key in [
        "test_execution_verified",
        "remote_ci_verified",
        "external_action_authorized",
    ] {
        assert_eq!(r[key], false);
    }
    blocked(run(&[
        "project-gate",
        "--project-root",
        f.path(),
        "--report",
        "docs/quality/report.json",
        "--stage",
        "merge",
    ]));
    fs::write(f.0.join("source.txt"), "changed source").unwrap();
    blocked(run(&args));
}
#[test]
fn unknown_duplicate_and_invalid_stage_arguments_block() {
    for args in [
        vec!["unknown"],
        vec!["validate-package", "--root", ".", "--root", "."],
        vec!["route-proposal", "--input"],
        vec!["validate-package", "--execute", "yes"],
    ] {
        blocked(run(&args));
    }
}

#[test]
fn planning_cli_actual_work_review_fix_acceptance_journey() {
    let f = Fixture::new();
    let input = f.0.join("plan.json");
    let runtime = f.0.join("runtime.json");
    let event = f.0.join("event.json");
    let mut plan: Value = serde_json::from_str(include_str!("../docs/examples/plan.json")).unwrap();
    let state: Value = serde_json::from_str(include_str!("../docs/examples/runtime.json")).unwrap();
    fs::write(&input, plan.to_string()).unwrap();
    fs::write(&runtime, state.to_string()).unwrap();
    let summary = success(run(&["validate-plan", "--input", input.to_str().unwrap()]));
    assert_eq!(summary["reported_evidence_verified"], false);
    let wave = success(run(&[
        "next-wave",
        "--input",
        input.to_str().unwrap(),
        "--runtime",
        runtime.to_str().unwrap(),
    ]));
    assert_eq!(wave["selected_tasks"], json!(["build"]));
    assert_eq!(wave["execution_performed"], false);
    plan["tasks"][0]["title"] = json!("Build | <script>\n[unsafe](https://invalid.example)");
    fs::write(&input, plan.to_string()).unwrap();
    let chart = run(&["render-chart", "--input", input.to_str().unwrap()]);
    assert!(chart.status.success());
    let chart = String::from_utf8(chart.stdout).unwrap();
    assert!(chart.contains("&#124;") && chart.contains("&lt;script&gt;"));
    assert!(!chart.contains("<script>") && !chart.contains("[unsafe]"));
    assert_eq!(
        chart.lines().filter(|line| line.starts_with('|')).count(),
        3
    );

    for (from, to, actor) in [
        ("planned", "running", "coordinator"),
        ("running", "review", "builder"),
        ("review", "needs-fix", "reviewer"),
        ("needs-fix", "running", "coordinator"),
        ("running", "review", "builder"),
        ("review", "accepted", "reviewer"),
    ] {
        let mut update = json!({"format_version":1,"project_id":plan["project_id"],
            "expected_revision":plan["revision"],"task_id":"build","actor":actor,"from":from,"to":to});
        if to == "needs-fix" {
            update["findings"] = json!(["Synthetic review finding"]);
        }
        if from == "needs-fix" {
            update["source_revision"] = json!("example-source-2");
            update["findings"] = json!([]);
        }
        if to == "review" {
            update["checks"] = json!([{"id":"qa","status":"passed","source_revision":plan["tasks"][0]["source_revision"]}]);
            update["artifacts"] = json!(["docs/quality/synthetic.txt"]);
        }
        if to == "accepted" {
            update["review"] = json!({"actor":"reviewer","status":"passed","source_revision":plan["tasks"][0]["source_revision"]});
            let mut skipped = update.clone();
            skipped["checks"] = json!([{"id":"qa","status":"skipped","source_revision":plan["tasks"][0]["source_revision"]}]);
            fs::write(&event, skipped.to_string()).unwrap();
            blocked(run(&[
                "apply-transition",
                "--input",
                input.to_str().unwrap(),
                "--event",
                event.to_str().unwrap(),
            ]));
        }
        fs::write(&event, update.to_string()).unwrap();
        let result = success(run(&[
            "apply-transition",
            "--input",
            input.to_str().unwrap(),
            "--event",
            event.to_str().unwrap(),
        ]));
        assert_eq!(result["external_action_authorized"], false);
        assert_eq!(result["reported_evidence_verified"], false);
        plan = result["plan"].clone();
        fs::write(&input, plan.to_string()).unwrap();
        blocked(run(&[
            "apply-transition",
            "--input",
            input.to_str().unwrap(),
            "--event",
            event.to_str().unwrap(),
        ]));
    }
    assert_eq!(plan["revision"], 6);
    assert_eq!(plan["tasks"][0]["status"], "accepted");
    success(run(&["validate-plan", "--input", input.to_str().unwrap()]));
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(&input).unwrap()).unwrap(),
        plan
    );
}

#[test]
fn planning_cli_blocks_bad_graph_runtime_and_bounded_json() {
    let f = Fixture::new();
    let input = f.0.join("plan.json");
    let runtime = f.0.join("runtime.json");
    let original: Value = serde_json::from_str(include_str!("../docs/examples/plan.json")).unwrap();
    let mut cycle = original.clone();
    cycle["tasks"][0]["depends_on"] = json!(["build"]);
    fs::write(&input, cycle.to_string()).unwrap();
    blocked(run(&["validate-plan", "--input", input.to_str().unwrap()]));
    fs::write(&input, original.to_string()).unwrap();
    let mut state: Value =
        serde_json::from_str(include_str!("../docs/examples/runtime.json")).unwrap();
    for field in ["available_capabilities", "active_tasks"] {
        state[field] = json!(["unknown"]);
        fs::write(&runtime, state.to_string()).unwrap();
        blocked(run(&[
            "next-wave",
            "--input",
            input.to_str().unwrap(),
            "--runtime",
            runtime.to_str().unwrap(),
        ]));
        state[field] = json!([]);
    }
    for text in [
        r#"{"format_version":1,"format_version":1}"#,
        r#"{"unused":NaN}"#,
    ] {
        fs::write(&input, text).unwrap();
        blocked(run(&["validate-plan", "--input", input.to_str().unwrap()]));
    }
    fs::write(&input, vec![b' '; 1024 * 1024 + 1]).unwrap();
    blocked(run(&["validate-plan", "--input", input.to_str().unwrap()]));
}
