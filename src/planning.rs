//! Deterministic planning over reported state, with no dispatch or persistence.
use crate::Result;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const STATES: [&str; 7] = [
    "planned",
    "running",
    "review",
    "needs-fix",
    "accepted",
    "blocked",
    "cancelled",
];
const LIMIT: usize = 1_000;

struct Task<'a> {
    id: &'a str,
    title: &'a str,
    owner: &'a str,
    status: &'a str,
    dependencies: Vec<&'a str>,
    capabilities: Vec<&'a str>,
    decisions: Vec<&'a str>,
    reads: Vec<&'a str>,
    writes: Vec<&'a str>,
    source: &'a str,
    required_checks: Vec<&'a str>,
    checks: BTreeMap<&'a str, (&'a str, &'a str)>,
    findings: Vec<&'a str>,
    artifacts: Vec<&'a str>,
    review: Option<(&'a str, &'a str, &'a str)>,
}

struct Plan<'a> {
    project: &'a str,
    revision: u64,
    insensitive: bool,
    decisions: BTreeMap<&'a str, &'a str>,
    capabilities: BTreeSet<&'a str>,
    tasks: BTreeMap<&'a str, Task<'a>>,
}

fn object(value: &Value) -> Result<&serde_json::Map<String, Value>> {
    value.as_object().ok_or_else(|| "Expected an object".into())
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| "Required string field missing".into())
}

fn identity(value: &str) -> Result<&str> {
    if value.is_empty()
        || value.len() > 128
        || !value.as_bytes()[0].is_ascii_alphanumeric()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_.".contains(&byte))
    {
        return Err("Identity must be an ASCII slug of at most 128 bytes".into());
    }
    Ok(value)
}

fn id<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    identity(string(value, field)?)
}

fn number(value: &Value, field: &str) -> Result<u64> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| "Required unsigned integer missing".into())
}

fn array<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>> {
    let items = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or("Required array missing")?;
    if items.len() > LIMIT {
        return Err("Array exceeds the supported item count".into());
    }
    Ok(items)
}

fn text(value: &str, maximum: usize) -> Result<&str> {
    if value.trim().is_empty() || value.len() > maximum || value.contains('\0') {
        return Err("Text is empty, oversized, or contains NUL".into());
    }
    Ok(value)
}

fn path(value: &str) -> Result<&str> {
    text(value, 512)?;
    if value.starts_with('/')
        || value
            .chars()
            .any(|ch| ch.is_control() || "\\:*?[]".contains(ch))
        || value.split('/').any(|part| {
            part.is_empty() || part == "." || part == ".." || part.ends_with(['.', ' '])
        })
    {
        return Err("Scopes must be concrete relative paths without traversal or wildcards".into());
    }
    Ok(value)
}

fn strings<'a>(
    value: &'a Value,
    field: &str,
    validator: fn(&str) -> Result<&str>,
) -> Result<Vec<&'a str>> {
    let mut found = BTreeSet::new();
    let mut output = Vec::new();
    for item in array(value, field)? {
        let item = validator(item.as_str().ok_or("Array items must be strings")?)?;
        if !found.insert(item) {
            return Err("Duplicate array identity or path".into());
        }
        output.push(item);
    }
    Ok(output)
}

fn parse(value: &Value) -> Result<Plan<'_>> {
    object(value)?;
    if number(value, "format_version")? != 1 {
        return Err("Unsupported plan format".into());
    }
    let project = id(value, "project_id")?;
    let revision = number(value, "revision")?;
    let insensitive = match string(value, "filesystem_case")? {
        "sensitive" => false,
        "insensitive" => true,
        _ => return Err("Filesystem case semantics must be explicit".into()),
    };
    let mut decisions = BTreeMap::new();
    for decision in array(value, "decisions")? {
        object(decision)?;
        let decision_id = id(decision, "id")?;
        let status = string(decision, "status")?;
        if !["open", "resolved"].contains(&status)
            || decisions.insert(decision_id, status).is_some()
        {
            return Err("Invalid or duplicate decision".into());
        }
    }
    let mut capabilities = BTreeSet::new();
    for capability in array(value, "capabilities")? {
        object(capability)?;
        if !["native-agent", "skill", "plugin", "mcp", "a2a"].contains(&string(capability, "kind")?)
            || !capabilities.insert(id(capability, "id")?)
        {
            return Err("Invalid or duplicate capability".into());
        }
    }
    let mut tasks = BTreeMap::new();
    let mut edges = 0;
    let mut scopes = 0;
    let mut check_records = 0;
    for value in array(value, "tasks")? {
        object(value)?;
        let status = string(value, "status")?;
        if !STATES.contains(&status) {
            return Err("Unsupported task state".into());
        }
        id(value, "kind")?;
        let mut checks = BTreeMap::new();
        for check in array(value, "checks")? {
            check_records += 1;
            if check_records > 10_000 {
                return Err("Plan exceeds the supported check count".into());
            }
            object(check)?;
            let check_id = id(check, "id")?;
            let status = string(check, "status")?;
            if !["passed", "failed", "skipped", "unverified"].contains(&status)
                || checks
                    .insert(check_id, (status, id(check, "source_revision")?))
                    .is_some()
            {
                return Err("Invalid or duplicate check".into());
            }
        }
        let review = match value
            .get("review")
            .ok_or("Review must explicitly be null or an object")?
        {
            Value::Null => None,
            record => {
                object(record)?;
                let status = string(record, "status")?;
                if !["passed", "failed"].contains(&status) {
                    return Err("Unsupported review state".into());
                }
                Some((id(record, "actor")?, id(record, "source_revision")?, status))
            }
        };
        let findings = array(value, "findings")?
            .iter()
            .map(|finding| text(finding.as_str().ok_or("Finding must be text")?, 2048))
            .collect::<Result<Vec<_>>>()?;
        let task = Task {
            id: id(value, "id")?,
            title: text(string(value, "title")?, 256)?,
            owner: id(value, "owner")?,
            status,
            dependencies: strings(value, "depends_on", identity)?,
            capabilities: strings(value, "capabilities", identity)?,
            decisions: strings(value, "requires_decisions", identity)?,
            reads: strings(value, "reads", path)?,
            writes: strings(value, "writes", path)?,
            source: id(value, "source_revision")?,
            required_checks: strings(value, "required_checks", identity)?,
            checks,
            findings,
            artifacts: strings(value, "artifacts", path)?,
            review,
        };
        let task_scopes = task.reads.len() + task.writes.len();
        scopes += task_scopes;
        if task_scopes > 32 || scopes > 4096 {
            return Err("Plan exceeds the supported scope count".into());
        }
        if insensitive {
            for paths in [&task.reads, &task.writes, &task.artifacts] {
                let folded: BTreeSet<_> = paths.iter().map(|path| path.to_lowercase()).collect();
                if folded.len() != paths.len() {
                    return Err("Case-equivalent duplicate paths".into());
                }
            }
        }
        edges += task.dependencies.len();
        if edges > 10_000 {
            return Err("Plan exceeds the supported dependency count".into());
        }
        if tasks.insert(task.id, task).is_some() {
            return Err("Duplicate task identity".into());
        }
    }
    let plan = Plan {
        project,
        revision,
        insensitive,
        decisions,
        capabilities,
        tasks,
    };
    for task in plan.tasks.values() {
        if task
            .dependencies
            .iter()
            .any(|dependency| *dependency == task.id || !plan.tasks.contains_key(dependency))
        {
            return Err("Unknown or self-referential dependency".into());
        }
        if task
            .capabilities
            .iter()
            .any(|capability| !plan.capabilities.contains(capability))
            || task
                .decisions
                .iter()
                .any(|decision| !plan.decisions.contains_key(decision))
        {
            return Err("Task references an unknown capability or decision".into());
        }
    }
    let mut counts: BTreeMap<_, _> = plan
        .tasks
        .iter()
        .map(|(id, task)| (*id, task.dependencies.len()))
        .collect();
    let mut successors: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for task in plan.tasks.values() {
        for dependency in &task.dependencies {
            successors.entry(dependency).or_default().push(task.id);
        }
    }
    let mut ready: BTreeSet<_> = counts
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(*id))
        .collect();
    let mut visited = 0;
    while let Some(id) = ready.pop_first() {
        visited += 1;
        if let Some(next) = successors.get(id) {
            for successor in next {
                let count = counts
                    .get_mut(successor)
                    .ok_or("Invalid dependency graph")?;
                *count -= 1;
                if *count == 0 {
                    ready.insert(*successor);
                }
            }
        }
    }
    if visited != plan.tasks.len() {
        return Err("Cyclic task dependencies".into());
    }
    for task in plan.tasks.values().filter(|task| task.status == "accepted") {
        if !prerequisites(&plan, task).is_empty() {
            return Err("Accepted task has unresolved prerequisites".into());
        }
        if !task.findings.is_empty() {
            return Err("Accepted task has unresolved findings".into());
        }
        if task
            .required_checks
            .iter()
            .any(|id| task.checks.get(id) != Some(&("passed", task.source)))
        {
            return Err("Accepted task lacks current passed required checks".into());
        }
        if !task.review.is_some_and(|(actor, source, status)| {
            actor != task.owner && source == task.source && status == "passed"
        }) {
            return Err("Accepted task lacks reported independent current review".into());
        }
    }
    Ok(plan)
}

fn flags() -> Value {
    json!({"execution_performed": false, "external_action_authorized": false, "reported_evidence_verified": false})
}

fn with_flags(mut result: Value) -> Value {
    result
        .as_object_mut()
        .expect("Object result")
        .extend(flags().as_object().expect("Object flags").clone());
    result
}

/// Validate reported plan claims, not execution, agent identity or evidence truth.
pub fn validate(value: &Value) -> Result<Value> {
    let plan = parse(value)?;
    Ok(with_flags(
        json!({"record_validation": "passed", "project_id": plan.project,
        "revision": plan.revision, "task_count": plan.tasks.len()}),
    ))
}

fn prerequisites(plan: &Plan<'_>, task: &Task<'_>) -> Vec<&'static str> {
    let mut reasons = Vec::new();
    if task
        .dependencies
        .iter()
        .any(|id| plan.tasks[id].status != "accepted")
    {
        reasons.push("dependencies-not-accepted");
    }
    if task
        .decisions
        .iter()
        .any(|id| plan.decisions[id] != "resolved")
    {
        reasons.push("decisions-not-resolved");
    }
    reasons
}

fn overlaps(left: &str, right: &str, insensitive: bool) -> bool {
    let left = if insensitive {
        left.to_lowercase()
    } else {
        left.to_owned()
    };
    let right = if insensitive {
        right.to_lowercase()
    } else {
        right.to_owned()
    };
    left == right
        || left
            .strip_prefix(&right)
            .is_some_and(|suffix| suffix.starts_with('/'))
        || right
            .strip_prefix(&left)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn conflicts(left: &Task<'_>, right: &Task<'_>, insensitive: bool) -> bool {
    left.writes.iter().any(|write| {
        right
            .reads
            .iter()
            .chain(&right.writes)
            .any(|scope| overlaps(write, scope, insensitive))
    }) || right.writes.iter().any(|write| {
        left.reads
            .iter()
            .any(|scope| overlaps(write, scope, insensitive))
    })
}

/// Select an ID-sorted feasible wave using explicit runtime availability and free worker slots.
pub fn next_wave(value: &Value, runtime: &Value) -> Result<Value> {
    let plan = parse(value)?;
    object(runtime)?;
    if number(runtime, "format_version")? != 1
        || id(runtime, "project_id")? != plan.project
        || number(runtime, "revision")? != plan.revision
    {
        return Err("Runtime identity or revision does not match the plan".into());
    }
    let slots = number(runtime, "free_worker_slots")?;
    if slots > 32 {
        return Err("Worker slot count exceeds the supported bound".into());
    }
    let available: BTreeSet<_> = strings(runtime, "available_capabilities", identity)?
        .into_iter()
        .collect();
    if available.iter().any(|id| !plan.capabilities.contains(id)) {
        return Err("Runtime lists an unknown capability".into());
    }
    let active: BTreeSet<_> = strings(runtime, "active_tasks", identity)?
        .into_iter()
        .collect();
    if active.iter().any(|id| {
        !plan
            .tasks
            .get(id)
            .is_some_and(|task| ["running", "review"].contains(&task.status))
    }) {
        return Err("Runtime active task is unknown or not running/reviewing".into());
    }
    if plan
        .tasks
        .values()
        .any(|task| task.status == "running" && !active.contains(task.id))
    {
        return Err("Runtime must include every running task".into());
    }
    let mut selected: Vec<&str> = Vec::new();
    let mut records = Vec::new();
    for task in plan.tasks.values() {
        let mut reasons = Vec::new();
        if !["planned", "needs-fix"].contains(&task.status) {
            reasons.push("state-not-schedulable");
        }
        reasons.extend(prerequisites(&plan, task));
        if task.capabilities.iter().any(|id| !available.contains(id)) {
            reasons.push("capabilities-unavailable");
        }
        if active
            .iter()
            .any(|id| conflicts(task, &plan.tasks[id], plan.insensitive))
        {
            reasons.push("scope-conflicts-with-active-task");
        }
        if selected
            .iter()
            .any(|id| conflicts(task, &plan.tasks[id], plan.insensitive))
        {
            reasons.push("scope-conflicts-with-selected-task");
        }
        if selected.len() >= slots as usize {
            reasons.push("no-free-worker-slot");
        }
        let decision = if reasons.is_empty() {
            selected.push(task.id);
            "selected"
        } else {
            "waiting"
        };
        records.push(json!({"task_id": task.id, "decision": decision, "reasons": reasons}));
    }
    Ok(with_flags(
        json!({"project_id": plan.project, "revision": plan.revision,
        "selected_tasks": selected, "tasks": records, "free_worker_slots": slots}),
    ))
}

fn escape(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        match ch {
            '\n' | '\r' | '\t' => output.push_str("&#32;"),
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '|' => output.push_str("&#124;"),
            '`' => output.push_str("&#96;"),
            '[' => output.push_str("&#91;"),
            ']' => output.push_str("&#93;"),
            '\\' => output.push_str("&#92;"),
            '*' => output.push_str("&#42;"),
            '_' => output.push_str("&#95;"),
            '(' => output.push_str("&#40;"),
            ')' => output.push_str("&#41;"),
            '!' => output.push_str("&#33;"),
            '#' => output.push_str("&#35;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            ch if ch.is_control() => output.push_str("&#32;"),
            _ => output.push(ch),
        }
    }
    output
}

/// Render escaped Markdown. All QA/review text describes reported records only.
pub fn render_chart(value: &Value) -> Result<String> {
    let plan = parse(value)?;
    let mut chart = String::from(
        "| Task | Owner | Dependencies | State | Artifacts | Reported review / QA | Next action |\n| --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for task in plan.tasks.values() {
        let review = task
            .review
            .map(|(actor, source, status)| {
                if source != task.source {
                    "stale"
                } else if actor == task.owner {
                    "unverified"
                } else {
                    status
                }
            })
            .unwrap_or("unverified");
        let passed = task
            .required_checks
            .iter()
            .filter(|id| task.checks.get(**id) == Some(&("passed", task.source)))
            .count();
        let next = match task.status {
            "planned" | "needs-fix" => {
                if prerequisites(&plan, task).is_empty() {
                    "check runtime availability and scopes"
                } else {
                    "resolve prerequisites"
                }
            }
            "running" => "inspect returned output",
            "review" => "review current output and checks",
            "blocked" => "resolve blocker",
            "accepted" => "continue dependent tasks",
            _ => "no action",
        };
        let columns = [
            format!("{}: {}", task.id, task.title),
            task.owner.to_owned(),
            task.dependencies.join(", "),
            task.status.to_owned(),
            task.artifacts.join(", "),
            format!(
                "review {review}; required checks {passed}/{}",
                task.required_checks.len()
            ),
            next.to_owned(),
        ];
        chart.push_str("| ");
        chart.push_str(
            &columns
                .iter()
                .map(|column| escape(column))
                .collect::<Vec<_>>()
                .join(" | "),
        );
        chart.push_str(" |\n");
    }
    chart.push_str("\nReported state only; execution, evidence, reviewer identity and external authorization remain unverified.\n");
    Ok(chart)
}

fn allowed(from: &str, to: &str) -> bool {
    match from {
        "planned" => ["running", "blocked", "cancelled"].contains(&to),
        "running" => ["review", "needs-fix", "blocked"].contains(&to),
        "review" => ["accepted", "needs-fix", "blocked"].contains(&to),
        "needs-fix" => ["running", "blocked", "cancelled"].contains(&to),
        "blocked" => ["planned", "cancelled"].contains(&to),
        "accepted" => to == "needs-fix",
        _ => false,
    }
}

/// Apply an explicit state event; caller owns saving and actual authorization checks.
pub fn transition(value: &Value, event: &Value) -> Result<Value> {
    let plan = parse(value)?;
    object(event)?;
    if number(event, "format_version")? != 1
        || id(event, "project_id")? != plan.project
        || number(event, "expected_revision")? != plan.revision
    {
        return Err("Transition identity or revision does not match the plan".into());
    }
    let task_id = id(event, "task_id")?;
    let task = plan
        .tasks
        .get(task_id)
        .ok_or("Transition references an unknown task")?;
    let actor = id(event, "actor")?;
    let from = string(event, "from")?;
    let to = string(event, "to")?;
    if from != task.status || !allowed(from, to) {
        return Err("Unsupported or stale task transition".into());
    }
    if to == "running" && !prerequisites(&plan, task).is_empty() {
        return Err("Running task has unresolved prerequisites".into());
    }
    if to == "review" && actor != task.owner {
        return Err("Only the recorded owner may submit output for review".into());
    }
    if from == "accepted" {
        let mut reached = BTreeSet::from([task_id]);
        let mut pending = BTreeSet::from([task_id]);
        while let Some(parent) = pending.pop_first() {
            for descendant in plan
                .tasks
                .values()
                .filter(|candidate| candidate.dependencies.contains(&parent))
            {
                if descendant.status == "accepted" {
                    return Err(
                        "Invalidate accepted downstream tasks before reopening this prerequisite"
                            .into(),
                    );
                }
                if reached.insert(descendant.id) {
                    pending.insert(descendant.id);
                }
            }
        }
    }
    let revision = plan
        .revision
        .checked_add(1)
        .ok_or("Plan revision overflow")?;
    let mut updated = value.clone();
    updated["revision"] = json!(revision);
    let target = updated["tasks"]
        .as_array_mut()
        .ok_or("Task array missing")?
        .iter_mut()
        .find(|candidate| candidate["id"].as_str() == Some(task_id))
        .ok_or("Task missing")?;
    for field in [
        "checks",
        "review",
        "findings",
        "artifacts",
        "source_revision",
    ] {
        if let Some(replacement) = event.get(field) {
            target[field] = replacement.clone();
        }
    }
    let invalidate_evidence = target["source_revision"].as_str() != Some(task.source)
        || to == "needs-fix"
        || (from == "needs-fix" && to == "running");
    if invalidate_evidence {
        if event.get("checks").is_some() || event.get("review").is_some() {
            return Err(
                "An evidence-invalidating event must not supply checks or review; reverify separately"
                    .into(),
            );
        }
        target["checks"] = json!([]);
        target["review"] = Value::Null;
    }
    target["status"] = json!(to);
    let revised = parse(&updated)?;
    if to == "accepted" {
        let review_actor = revised.tasks[task_id]
            .review
            .map(|review| review.0)
            .ok_or("Acceptance requires a review")?;
        if actor != review_actor || actor == task.owner {
            return Err("Acceptance actor must match the recorded independent reviewer".into());
        }
    }
    Ok(with_flags(
        json!({"project_id": plan.project, "revision": revision,
        "task_id": task_id, "from": from, "to": to, "plan": updated}),
    ))
}
