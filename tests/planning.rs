use agent_orchestrator::planning::{next_wave, render_chart, transition, validate};
use serde_json::{Value, json};

fn task(id: &str, dependencies: &[&str]) -> Value {
    json!({"id": id, "title": format!("Implement {id}"), "owner": "builder", "kind": "implementation",
        "status": "planned", "depends_on": dependencies, "capabilities": ["native-builder"],
        "requires_decisions": ["requirements"], "reads": [], "writes": [format!("src/{id}")],
        "source_revision": "source-1", "required_checks": ["qa"], "checks": [], "findings": [],
        "artifacts": [], "review": null})
}

fn plan(tasks: Vec<Value>) -> Value {
    json!({"format_version": 1, "project_id": "example", "revision": 0, "filesystem_case": "sensitive",
        "decisions": [{"id": "requirements", "status": "resolved"}],
        "capabilities": [{"id": "native-builder", "kind": "native-agent"}], "tasks": tasks})
}

fn runtime(plan: &Value, slots: u64, active: &[&str]) -> Value {
    json!({"format_version": 1, "project_id": plan["project_id"], "revision": plan["revision"],
        "free_worker_slots": slots, "available_capabilities": ["native-builder"], "active_tasks": active})
}

fn accepted(mut task: Value) -> Value {
    task["status"] = json!("accepted");
    task["checks"] =
        json!([{"id": "qa", "status": "passed", "source_revision": task["source_revision"]}]);
    task["review"] = json!({"actor": "reviewer", "status": "passed", "source_revision": task["source_revision"]});
    task
}

fn event(plan: &Value, id: &str, from: &str, to: &str, actor: &str) -> Value {
    json!({"format_version": 1, "project_id": plan["project_id"], "expected_revision": plan["revision"],
        "task_id": id, "actor": actor, "from": from, "to": to})
}

fn false_flags(value: &Value) {
    for field in [
        "execution_performed",
        "external_action_authorized",
        "reported_evidence_verified",
    ] {
        assert_eq!(value[field], false);
    }
}

#[test]
fn lifecycle_routes_reviews_and_accepts_reported_state_without_execution() {
    let mut current = plan(vec![task("build", &[]), task("release", &["build"])]);
    let summary = validate(&current).unwrap();
    assert_eq!(summary["record_validation"], "passed");
    false_flags(&summary);
    let wave = next_wave(&current, &runtime(&current, 2, &[])).unwrap();
    assert_eq!(wave["selected_tasks"], json!(["build"]));
    false_flags(&wave);
    let start = transition(
        &current,
        &event(&current, "build", "planned", "running", "coordinator"),
    )
    .unwrap();
    false_flags(&start);
    current = start["plan"].clone();
    let mut output = event(&current, "build", "running", "review", "builder");
    output["checks"] = json!([{"id": "qa", "status": "passed", "source_revision": "source-1"}]);
    output["artifacts"] = json!(["docs/quality/build.txt"]);
    current = transition(&current, &output).unwrap()["plan"].clone();
    let mut accept = event(&current, "build", "review", "accepted", "reviewer");
    accept["review"] =
        json!({"actor": "reviewer", "source_revision": "source-1", "status": "passed"});
    current = transition(&current, &accept).unwrap()["plan"].clone();
    assert_eq!(current["revision"], 3);
    assert_eq!(
        next_wave(&current, &runtime(&current, 1, &[])).unwrap()["selected_tasks"],
        json!(["release"])
    );
}

#[test]
fn invalid_graphs_identities_and_required_fields_are_rejected() {
    let original = plan(vec![task("first", &[]), task("second", &["first"])]);
    for field in [
        "format_version",
        "project_id",
        "revision",
        "filesystem_case",
        "decisions",
        "capabilities",
        "tasks",
    ] {
        let mut invalid = original.clone();
        invalid.as_object_mut().unwrap().remove(field);
        assert!(validate(&invalid).is_err(), "missing {field}");
    }
    for field in [
        "id",
        "title",
        "owner",
        "kind",
        "status",
        "depends_on",
        "capabilities",
        "requires_decisions",
        "reads",
        "writes",
        "source_revision",
        "required_checks",
        "checks",
        "findings",
        "artifacts",
        "review",
    ] {
        let mut invalid = original.clone();
        invalid["tasks"][0].as_object_mut().unwrap().remove(field);
        assert!(validate(&invalid).is_err(), "missing task {field}");
    }
    for identity in ["", "with spaces", "_prefix", "x/y", "é", &"a".repeat(129)] {
        let mut invalid = original.clone();
        invalid["tasks"][0]["id"] = json!(identity);
        assert!(validate(&invalid).is_err());
    }
    for dependencies in [
        json!(["unknown"]),
        json!(["first"]),
        json!(["second", "second"]),
        json!(["second"]),
    ] {
        let mut invalid = original.clone();
        invalid["tasks"][0]["depends_on"] = dependencies;
        assert!(validate(&invalid).is_err());
    }
    let mut invalid = original.clone();
    invalid["tasks"][1]["id"] = json!("first");
    assert!(validate(&invalid).is_err());
    for field in ["capabilities", "requires_decisions"] {
        let mut invalid = original.clone();
        invalid["tasks"][0][field] = json!(["unknown"]);
        assert!(validate(&invalid).is_err());
    }
    let mut invalid = original.clone();
    invalid["decisions"][0]["status"] = json!("assumed");
    assert!(validate(&invalid).is_err());
    invalid = original.clone();
    invalid["capabilities"][0]["kind"] = json!("invented");
    assert!(validate(&invalid).is_err());
}

#[test]
fn concrete_relative_scopes_are_required() {
    for scope in [
        "",
        "/root",
        "../src",
        "src/../other",
        "src/./file",
        "src//file",
        "src/",
        "C:/root",
        "src\\file",
        "src/*",
        "src/?",
        "src/[a]",
        "src/file:stream",
        "src/name.",
        "src/name ",
        "src/line\nbreak",
    ] {
        for field in ["reads", "writes", "artifacts"] {
            let mut invalid = plan(vec![task("first", &[])]);
            invalid["tasks"][0][field] = json!([scope]);
            assert!(validate(&invalid).is_err(), "invalid {field}: {scope:?}");
        }
    }
    let mut invalid = plan(vec![task("first", &[])]);
    invalid["filesystem_case"] = json!("insensitive");
    invalid["tasks"][0]["writes"] = json!(["src/File", "src/file"]);
    assert!(validate(&invalid).is_err());
}

#[test]
fn accepted_tasks_need_current_independent_review_checks_and_resolved_inputs() {
    let original = plan(vec![accepted(task("first", &[]))]);
    assert!(validate(&original).is_ok());
    for check in [
        json!([]),
        json!([{"id": "qa", "status": "skipped", "source_revision": "source-1"}]),
        json!([{"id": "qa", "status": "passed", "source_revision": "stale"}]),
    ] {
        let mut invalid = original.clone();
        invalid["tasks"][0]["checks"] = check;
        assert!(validate(&invalid).is_err());
    }
    for review in [
        Value::Null,
        json!({"actor": "builder", "status": "passed", "source_revision": "source-1"}),
        json!({"actor": "reviewer", "status": "failed", "source_revision": "source-1"}),
        json!({"actor": "reviewer", "status": "passed", "source_revision": "stale"}),
    ] {
        let mut invalid = original.clone();
        invalid["tasks"][0]["review"] = review;
        assert!(validate(&invalid).is_err());
    }
    let mut invalid = original.clone();
    invalid["tasks"][0]["findings"] = json!(["security issue"]);
    assert!(validate(&invalid).is_err());
    invalid = original.clone();
    invalid["decisions"][0]["status"] = json!("open");
    assert!(validate(&invalid).is_err());
    assert!(
        validate(&plan(vec![
            task("prerequisite", &[]),
            accepted(task("dependent", &["prerequisite"]))
        ]))
        .is_err()
    );
}

#[test]
fn scheduling_is_stable_capacity_bounded_and_respects_capability_availability() {
    let current = plan(vec![
        task("z-last", &[]),
        task("a-first", &[]),
        task("m-middle", &[]),
    ]);
    let wave = next_wave(&current, &runtime(&current, 2, &[])).unwrap();
    assert_eq!(wave["selected_tasks"], json!(["a-first", "m-middle"]));
    assert_eq!(
        next_wave(&current, &runtime(&current, 0, &[])).unwrap()["selected_tasks"],
        json!([])
    );
    let mut unavailable = runtime(&current, 3, &[]);
    unavailable["available_capabilities"] = json!([]);
    assert_eq!(
        next_wave(&current, &unavailable).unwrap()["selected_tasks"],
        json!([])
    );
    let mut blocked = current.clone();
    blocked["decisions"][0]["status"] = json!("open");
    assert_eq!(
        next_wave(&blocked, &runtime(&blocked, 3, &[])).unwrap()["selected_tasks"],
        json!([])
    );
}

#[test]
fn scheduling_read_write_scopes_include_active_and_selected_tasks() {
    let mut active = task("active", &[]);
    active["status"] = json!("running");
    active["writes"] = json!(["src/shared"]);
    let mut reader = task("reader", &[]);
    reader["reads"] = json!(["src/shared/file.rs"]);
    reader["writes"] = json!([]);
    let free = task("free", &[]);
    let current = plan(vec![active, reader, free]);
    let wave = next_wave(&current, &runtime(&current, 3, &["active"])).unwrap();
    assert_eq!(wave["selected_tasks"], json!(["free"]));
    assert!(next_wave(&current, &runtime(&current, 3, &[])).is_err());
    let mut first = task("a-reader", &[]);
    first["reads"] = json!(["src/shared"]);
    first["writes"] = json!([]);
    let mut second = first.clone();
    second["id"] = json!("b-reader");
    let mut writer = task("c-writer", &[]);
    writer["writes"] = json!(["src/shared/file.rs"]);
    let current = plan(vec![writer, second, first]);
    assert_eq!(
        next_wave(&current, &runtime(&current, 3, &[])).unwrap()["selected_tasks"],
        json!(["a-reader", "b-reader"])
    );
}

#[test]
fn scope_prefixes_and_explicit_case_semantics_are_respected() {
    let mut first = task("a", &[]);
    first["writes"] = json!(["Src/shared"]);
    let mut second = task("b", &[]);
    second["writes"] = json!(["src/shared/file"]);
    let mut third = task("c", &[]);
    third["writes"] = json!(["Src/shared-extra"]);
    let mut current = plan(vec![first, second, third]);
    assert_eq!(
        next_wave(&current, &runtime(&current, 3, &[])).unwrap()["selected_tasks"],
        json!(["a", "b", "c"])
    );
    current["filesystem_case"] = json!("insensitive");
    assert_eq!(
        next_wave(&current, &runtime(&current, 3, &[])).unwrap()["selected_tasks"],
        json!(["a", "c"])
    );
}

#[test]
fn runtime_requires_explicit_valid_identity_revision_capacity_and_active_state() {
    let current = plan(vec![task("first", &[])]);
    let original = runtime(&current, 1, &[]);
    for field in [
        "format_version",
        "project_id",
        "revision",
        "free_worker_slots",
        "available_capabilities",
        "active_tasks",
    ] {
        let mut invalid = original.clone();
        invalid.as_object_mut().unwrap().remove(field);
        assert!(next_wave(&current, &invalid).is_err());
    }
    for value in [json!(true), json!(1.0), json!(-1), json!(33), json!("2")] {
        let mut invalid = original.clone();
        invalid["free_worker_slots"] = value;
        assert!(next_wave(&current, &invalid).is_err());
    }
    for (field, value) in [
        ("project_id", json!("other")),
        ("revision", json!(1)),
        ("available_capabilities", json!(["unknown"])),
        (
            "available_capabilities",
            json!(["native-builder", "native-builder"]),
        ),
        ("active_tasks", json!(["unknown"])),
        ("active_tasks", json!(["first"])),
    ] {
        let mut invalid = original.clone();
        invalid[field] = value;
        assert!(next_wave(&current, &invalid).is_err());
    }
    let mut reviewing = current.clone();
    reviewing["tasks"][0]["status"] = json!("review");
    assert!(next_wave(&reviewing, &runtime(&reviewing, 1, &[])).is_ok());
    assert!(next_wave(&reviewing, &runtime(&reviewing, 1, &["first"])).is_ok());
}

#[test]
fn transitions_reject_stale_events_invalid_edges_and_review_identity_claims() {
    let original = plan(vec![task("first", &[])]);
    let valid = event(&original, "first", "planned", "running", "builder");
    for field in [
        "format_version",
        "project_id",
        "expected_revision",
        "task_id",
        "actor",
        "from",
        "to",
    ] {
        let mut invalid = valid.clone();
        invalid.as_object_mut().unwrap().remove(field);
        assert!(transition(&original, &invalid).is_err());
    }
    for (field, value) in [
        ("project_id", json!("other")),
        ("expected_revision", json!(1)),
        ("task_id", json!("unknown")),
        ("from", json!("review")),
        ("to", json!("planned")),
        ("to", json!("accepted")),
    ] {
        let mut invalid = valid.clone();
        invalid[field] = value;
        assert!(transition(&original, &invalid).is_err());
    }
    let mut current = original.clone();
    current["tasks"][0]["status"] = json!("running");
    assert!(
        transition(
            &current,
            &event(&current, "first", "running", "review", "other")
        )
        .is_err()
    );
    current["tasks"][0] = accepted(task("first", &[]));
    current["tasks"][0]["status"] = json!("review");
    assert!(
        transition(
            &current,
            &event(&current, "first", "review", "accepted", "builder")
        )
        .is_err()
    );
    assert!(
        transition(
            &current,
            &event(&current, "first", "review", "accepted", "other")
        )
        .is_err()
    );
    assert!(
        transition(
            &current,
            &event(&current, "first", "review", "accepted", "reviewer")
        )
        .is_ok()
    );
    current["revision"] = json!(u64::MAX);
    assert!(
        transition(
            &current,
            &event(&current, "first", "review", "needs-fix", "coordinator")
        )
        .is_err()
    );
    let mut unresolved = original.clone();
    unresolved["decisions"][0]["status"] = json!("open");
    assert!(
        transition(
            &unresolved,
            &event(&unresolved, "first", "planned", "running", "builder")
        )
        .is_err()
    );
}

#[test]
fn source_changes_clear_checks_and_review_and_cannot_restore_them_in_same_event() {
    let mut current = plan(vec![accepted(task("first", &[]))]);
    current["tasks"][0]["status"] = json!("review");
    let mut change = event(&current, "first", "review", "needs-fix", "coordinator");
    change["source_revision"] = json!("source-2");
    let updated = transition(&current, &change).unwrap();
    assert_eq!(updated["plan"]["tasks"][0]["checks"], json!([]));
    assert_eq!(updated["plan"]["tasks"][0]["review"], Value::Null);
    for field in ["checks", "review"] {
        let mut spoof = change.clone();
        spoof[field] = current["tasks"][0][field].clone();
        assert!(transition(&current, &spoof).is_err());
    }
    change["source_revision"] = json!("bad source name");
    assert!(transition(&current, &change).is_err());
}

#[test]
fn accepted_downstream_must_be_invalidated_from_leaves_before_prerequisites() {
    let mut current = plan(vec![
        accepted(task("parent", &[])),
        accepted(task("child", &["parent"])),
        accepted(task("grandchild", &["child"])),
    ]);
    assert!(
        transition(
            &current,
            &event(&current, "parent", "accepted", "needs-fix", "coordinator")
        )
        .is_err()
    );
    assert!(
        transition(
            &current,
            &event(&current, "child", "accepted", "needs-fix", "coordinator")
        )
        .is_err()
    );
    current = transition(
        &current,
        &event(
            &current,
            "grandchild",
            "accepted",
            "needs-fix",
            "coordinator",
        ),
    )
    .unwrap()["plan"]
        .clone();
    current = transition(
        &current,
        &event(&current, "child", "accepted", "needs-fix", "coordinator"),
    )
    .unwrap()["plan"]
        .clone();
    current = transition(
        &current,
        &event(&current, "parent", "accepted", "needs-fix", "coordinator"),
    )
    .unwrap()["plan"]
        .clone();
    assert_eq!(current["revision"], 3);
    assert!(validate(&current).is_ok());
}

#[test]
fn markdown_chart_escapes_untrusted_text_and_keeps_single_rows() {
    let mut current = plan(vec![task("first", &[])]);
    current["tasks"][0]["title"] = json!("x |\n`code` <script>[link](https://evil.invalid) \\ end");
    let chart = render_chart(&current).unwrap();
    assert!(chart.contains("&#124;&#32;&#96;code&#96; &lt;script&gt;&#91;link&#93;"));
    assert!(chart.contains("&#92;"));
    assert!(!chart.contains("<script>"));
    assert!(!chart.contains("[link]"));
    assert_eq!(
        chart.lines().filter(|line| line.starts_with('|')).count(),
        3
    );
    assert!(chart.contains("Reported state only"));
}

#[test]
fn same_source_reopening_and_restarting_clear_evidence_until_new_qa_and_review() {
    let mut current = plan(vec![
        accepted(task("parent", &[])),
        accepted(task("child", &["parent"])),
    ]);
    current = transition(
        &current,
        &event(&current, "child", "accepted", "needs-fix", "coordinator"),
    )
    .unwrap()["plan"]
        .clone();
    current = transition(
        &current,
        &event(&current, "parent", "accepted", "needs-fix", "coordinator"),
    )
    .unwrap()["plan"]
        .clone();
    for item in current["tasks"].as_array().unwrap() {
        assert_eq!(item["checks"], json!([]));
        assert_eq!(item["review"], Value::Null);
        assert_eq!(item["source_revision"], "source-1");
    }
    let restart = event(&current, "parent", "needs-fix", "running", "builder");
    for field in ["checks", "review"] {
        let mut spoof = restart.clone();
        spoof[field] = accepted(task("parent", &[]))[field].clone();
        assert!(transition(&current, &spoof).is_err());
    }
    current = transition(&current, &restart).unwrap()["plan"].clone();
    current = transition(
        &current,
        &event(&current, "parent", "running", "review", "builder"),
    )
    .unwrap()["plan"]
        .clone();
    let mut accept = event(&current, "parent", "review", "accepted", "reviewer");
    assert!(transition(&current, &accept).is_err());
    accept["review"] =
        json!({"actor": "reviewer", "source_revision": "source-1", "status": "passed"});
    assert!(
        transition(&current, &accept).is_err(),
        "Old same-source QA must not survive reopening"
    );
    let mut valid = accept;
    valid["checks"] = json!([{"id": "qa", "status": "passed", "source_revision": "source-1"}]);
    assert!(transition(&current, &valid).is_ok());
}

#[test]
fn entering_needs_fix_invalidates_evidence_and_rejects_evidence_replacements() {
    for state in ["accepted", "running", "review"] {
        let mut current = plan(vec![accepted(task("first", &[]))]);
        current["tasks"][0]["status"] = json!(state);
        let invalidation = event(&current, "first", state, "needs-fix", "coordinator");
        for field in ["checks", "review"] {
            let mut spoof = invalidation.clone();
            spoof[field] = current["tasks"][0][field].clone();
            assert!(transition(&current, &spoof).is_err());
        }
        let revised = transition(&current, &invalidation).unwrap();
        assert_eq!(revised["plan"]["tasks"][0]["checks"], json!([]));
        assert_eq!(revised["plan"]["tasks"][0]["review"], Value::Null);
    }
}

#[test]
fn chart_never_presents_stale_or_self_review_as_passed() {
    let mut current = plan(vec![accepted(task("first", &[]))]);
    current["tasks"][0]["status"] = json!("review");
    current["tasks"][0]["review"]["source_revision"] = json!("stale");
    let stale = render_chart(&current).unwrap();
    assert!(stale.contains("review stale"));
    assert!(!stale.contains("review passed"));
    current["tasks"][0]["review"]["source_revision"] = json!("source-1");
    current["tasks"][0]["review"]["actor"] = json!("builder");
    let self_review = render_chart(&current).unwrap();
    assert!(self_review.contains("review unverified"));
    assert!(!self_review.contains("review passed"));
}

#[test]
fn deterministic_bounds_limit_scopes_tasks_edges_and_checks() {
    let mut oversized = plan(vec![task("first", &[])]);
    oversized["tasks"][0]["reads"] = json!(
        (0..32)
            .map(|index| format!("read/{index}"))
            .collect::<Vec<_>>()
    );
    assert!(validate(&oversized).is_err());
    let many_scopes = (0..129)
        .map(|index| {
            let mut item = task(&format!("task-{index}"), &[]);
            item["reads"] = json!(
                (0..31)
                    .map(|scope| format!("read/{scope}"))
                    .collect::<Vec<_>>()
            );
            item
        })
        .collect();
    assert!(validate(&plan(many_scopes)).is_err());
    assert!(
        validate(&plan(
            (0..1001)
                .map(|index| task(&format!("task-{index}"), &[]))
                .collect()
        ))
        .is_err()
    );
    let many_edges = (0..142)
        .map(|index| {
            let mut item = task(&format!("task-{index}"), &[]);
            item["depends_on"] = json!(
                (0..index)
                    .map(|prior| format!("task-{prior}"))
                    .collect::<Vec<_>>()
            );
            item
        })
        .collect();
    assert!(validate(&plan(many_edges)).is_err());
    let many_checks = (0..11).map(|index| {
        let mut item = task(&format!("task-{index}"), &[]);
        item["checks"] = json!((0..1000).map(|check| json!({"id": format!("check-{check}"), "status": "passed", "source_revision": "source-1"})).collect::<Vec<_>>());
        item
    }).collect();
    assert!(validate(&plan(many_checks)).is_err());
}
