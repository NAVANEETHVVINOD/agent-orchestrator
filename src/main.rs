use agent_orchestrator::{Result, gate, package, read_bounded, routing, strict_json};
use serde_json::{Value, json};
use std::{collections::HashMap, path::Path};

fn run() -> Result<Value> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .ok_or("Use validate-package, project-gate or route-proposal")?;
    let mut options = HashMap::new();
    let mut args = std::env::args().skip(2);
    while let Some(key) = args.next() {
        let permitted = match command.as_str() {
            "validate-package" => ["--root"].as_slice(),
            "route-proposal" => ["--input"].as_slice(),
            "project-gate" => ["--project-root", "--report", "--stage", "--snapshot"].as_slice(),
            _ => return Err("Unknown command".into()),
        };
        if !permitted.contains(&key.as_str()) || options.contains_key(&key) {
            return Err("Unknown or duplicate argument".into());
        }
        let value = if key == "--snapshot" {
            String::new()
        } else {
            args.next().ok_or("Argument value missing")?
        };
        options.insert(key, value);
    }
    let needed = |key: &str| {
        options
            .get(key)
            .map(String::as_str)
            .ok_or_else(|| "Required argument missing".to_string())
    };
    match command.as_str() {
        "validate-package" => package::validate(Path::new(needed("--root")?)),
        "route-proposal" => routing::evaluate(&strict_json::parse(&read_bounded(
            Path::new(needed("--input")?),
            1024 * 1024,
        )?)?),
        "project-gate" => {
            let root = Path::new(needed("--project-root")?)
                .canonicalize()
                .map_err(|_| "Project root unavailable")?;
            let current = gate::snapshot(&root)?;
            let stage = options
                .get("--stage")
                .map(String::as_str)
                .unwrap_or("pre-push");
            if !["pre-push", "merge", "deploy"].contains(&stage) {
                return Err("Unsupported gate stage".into());
            }
            if options.contains_key("--snapshot") {
                if options.contains_key("--report") {
                    return Err("Choose snapshot or report".into());
                }
                return Ok(current);
            }
            let path = gate::evidence_path(&root, needed("--report")?)?;
            gate::validate_report(
                &root,
                &strict_json::parse(&read_bounded(&path, 1024 * 1024)?)?,
                stage,
                &current,
            )
        }
        _ => Err("Unknown command".into()),
    }
}
fn main() {
    match run() {
        Ok(value) => println!(
            "{}",
            serde_json::to_string_pretty(&value).unwrap_or_default()
        ),
        Err(reason) => {
            eprintln!(
                "{}",
                json!({"record_validation":"blocked","reason":reason,"external_action_authorized":false})
            );
            std::process::exit(1);
        }
    }
}
