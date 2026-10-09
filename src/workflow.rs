//! Strict, declarative project workflow configuration validation.
//!
//! References are syntax-checked only. The host must discover live capabilities
//! and separately authorize every external action.
use crate::{Result, strict_json};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

const MAX_ROLES: usize = 64;
const MAX_SKILLS_PER_ROLE: usize = 16;
const MAX_GATES: usize = 16;

const REQUIRED_GATES: &[&str] = &[
    "requirements-confirmed",
    "independent-review",
    "evidence-review",
    "security-review",
    "end-to-end-qa",
    "documentation-update",
    "local-checks-before-push",
    "hosted-ci-before-merge",
];

const GATES: &[&str] = &[
    "requirements-confirmed",
    "independent-review",
    "evidence-review",
    "security-review",
    "end-to-end-qa",
    "documentation-update",
    "local-checks-before-push",
    "hosted-ci-before-merge",
    "accessibility-review",
    "privacy-review",
    "release-readiness",
];

fn exact_object<'a>(value: &'a Value, keys: &[&str]) -> Result<&'a Map<String, Value>> {
    let object = value
        .as_object()
        .ok_or_else(|| "Workflow object required".to_string())?;
    if object.len() != keys.len() || object.keys().any(|key| !keys.contains(&key.as_str())) {
        return Err("Workflow has unknown or missing fields".into());
    }
    Ok(object)
}

fn bounded_string(value: &Value, max: usize) -> Result<&str> {
    let text = value
        .as_str()
        .ok_or_else(|| "Workflow string required".to_string())?;
    let first = text.chars().next();
    let last = text.chars().next_back();
    if text.is_empty()
        || first.is_some_and(ecma_whitespace)
        || last.is_some_and(ecma_whitespace)
        || text.chars().take(max + 1).count() > max
    {
        return Err("Workflow string is empty, padded or too long".into());
    }
    Ok(text)
}

fn ecma_whitespace(character: char) -> bool {
    matches!(
        character,
        '\u{0009}'..='\u{000D}'
            | '\u{0020}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

fn numeric_one(value: &Value) -> bool {
    let Some(number) = value.as_number() else {
        return false;
    };
    let raw = number.to_string();
    if raw.starts_with('-') {
        return false;
    }
    let (coefficient, exponent) = raw
        .split_once(['e', 'E'])
        .map_or((raw.as_str(), 0_i64), |(coefficient, exponent)| {
            (coefficient, exponent.parse::<i64>().unwrap_or(i64::MAX))
        });
    if exponent == i64::MAX {
        return false;
    }
    let (whole, fractional) = coefficient.split_once('.').unwrap_or((coefficient, ""));
    let digits = format!("{whole}{fractional}");
    let Some(first_nonzero) = digits.find(|digit: char| digit != '0') else {
        return false;
    };
    let significant = &digits[first_nonzero..];
    let trimmed = significant.trim_end_matches('0');
    if trimmed != "1" {
        return false;
    }
    let trailing_zero_count = significant.len() - trimmed.len();
    exponent
        .checked_sub(fractional.len() as i64)
        .and_then(|scale| scale.checked_add(trailing_zero_count as i64))
        == Some(0)
}

fn valid_id(value: &str) -> bool {
    if value.len() > 64 {
        return false;
    }
    let mut bytes = value.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z'))
        && bytes
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"_-".contains(&byte))
}

fn valid_capability_ref(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.split('/').all(|segment| {
            let mut bytes = segment.bytes();
            matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9'))
                && bytes.all(|byte| byte.is_ascii_alphanumeric() || b"_.:@-".contains(&byte))
        })
}

fn string_list(value: &Value, max_items: usize, max_length: usize) -> Result<Vec<&str>> {
    let values = value
        .as_array()
        .ok_or_else(|| "Workflow list required".to_string())?;
    if values.is_empty() || values.len() > max_items {
        return Err("Workflow list has unsupported size".into());
    }
    values
        .iter()
        .map(|item| {
            let text = bounded_string(item, max_length)?;
            if !valid_id(text) {
                return Err("Workflow identifier is invalid".into());
            }
            Ok(text)
        })
        .collect()
}

fn unique(values: &[&str]) -> bool {
    let mut seen = HashSet::new();
    values.iter().all(|value| seen.insert(*value))
}

/// Validate parsed workflow structure. CLI/MCP inputs are bounded before parsing;
/// direct byte callers should use `validate_bytes` to enforce the 1 MiB limit.
pub fn validate(value: &Value) -> Result<Value> {
    let root = exact_object(
        value,
        &[
            "format_version",
            "project",
            "roles",
            "required_quality_gates",
        ],
    )?;
    if !root.get("format_version").is_some_and(numeric_one) {
        return Err("Unsupported workflow format version".into());
    }

    let project = exact_object(root.get("project").unwrap(), &["id", "name", "languages"])?;
    let project_id = bounded_string(project.get("id").unwrap(), 64)?;
    if !valid_id(project_id) {
        return Err("Project identifier is invalid".into());
    }
    bounded_string(project.get("name").unwrap(), 120)?;
    let languages = string_list(project.get("languages").unwrap(), 16, 32)?;
    if !unique(&languages) {
        return Err("Duplicate project language".into());
    }

    let roles = root
        .get("roles")
        .and_then(Value::as_object)
        .ok_or_else(|| "Workflow roles object required".to_string())?;
    if roles.is_empty() || roles.len() > MAX_ROLES {
        return Err("Workflow roles list has unsupported size".into());
    }
    for (id, role) in roles {
        if !valid_id(id) {
            return Err("Invalid workflow role ID".into());
        }
        let role = exact_object(role, &["profile", "skills"])?;
        let profile = bounded_string(role.get("profile").unwrap(), 128)?;
        if !valid_capability_ref(profile) {
            return Err("Profile reference syntax is invalid".into());
        }
        let skills = role
            .get("skills")
            .and_then(Value::as_array)
            .ok_or_else(|| "Workflow skills list required".to_string())?;
        if skills.len() > MAX_SKILLS_PER_ROLE {
            return Err("Too many skills for workflow role".into());
        }
        let skill_ids = skills
            .iter()
            .map(|skill| {
                let skill = bounded_string(skill, 128)?;
                if !valid_capability_ref(skill) {
                    return Err("Skill reference syntax is invalid".into());
                }
                Ok(skill)
            })
            .collect::<Result<Vec<_>>>()?;
        if !unique(&skill_ids) {
            return Err("Duplicate workflow skill ID".into());
        }
    }

    let gates = string_list(root.get("required_quality_gates").unwrap(), MAX_GATES, 64)?;
    if !unique(&gates) {
        return Err("Duplicate workflow quality gate".into());
    }
    if gates.iter().any(|gate| !GATES.contains(gate))
        || REQUIRED_GATES
            .iter()
            .any(|required| !gates.contains(required))
    {
        return Err("Unknown or missing required quality gate".into());
    }

    Ok(json!({
        "record_validation": "passed",
        "format_version": 1,
        "role_count": roles.len(),
        "required_quality_gates": gates,
        "profile_and_skill_reference_syntax_validated": true,
        "live_host_capabilities_discovered": false,
        "agents_dispatched": false,
        "tools_installed": false,
        "permissions_granted": false,
        "external_action_authorized": false
    }))
}

/// Parse untrusted bytes with duplicate-key and depth checks before validation.
pub fn validate_bytes(bytes: &[u8]) -> Result<Value> {
    if bytes.len() > 1024 * 1024 {
        return Err("Workflow record exceeds 1 MiB".into());
    }
    validate(&strict_json::parse(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Value {
        serde_json::from_str(include_str!("../examples/project-workflow.json")).unwrap()
    }

    #[test]
    fn bundled_workflow_validates_without_claiming_runtime_availability() {
        let result = validate(&sample()).unwrap();
        assert_eq!(result["record_validation"], "passed");
        assert_eq!(result["role_count"], 14);
        assert_eq!(result["live_host_capabilities_discovered"], false);
        assert_eq!(result["agents_dispatched"], false);
        assert_eq!(result["permissions_granted"], false);
    }

    #[test]
    fn malformed_duplicate_or_missing_references_are_rejected() {
        let mut workflow = sample();
        workflow["roles"]["business-analysis"]["profile"] = json!("../untrusted-profile");
        assert!(validate(&workflow).is_err());

        let mut workflow = sample();
        workflow["roles"]["business-analysis"]["skills"] = json!(["project-qa", "project-qa"]);
        assert!(validate(&workflow).is_err());

        let mut workflow = sample();
        workflow["roles"]["business-analysis"]["skills"] = json!("not-a-list");
        assert!(validate(&workflow).is_err());

        let mut workflow = sample();
        workflow["roles"]["business-analysis"]["profile"] = json!("community/third-party-agent");
        workflow["roles"]["business-analysis"]["skills"] = json!(["firecrawl/firecrawl"]);
        assert!(validate(&workflow).is_ok());

        workflow["roles"]["business-analysis"]["profile"] = json!("community/-agent");
        assert!(validate(&workflow).is_err());

        let mut workflow = sample();
        workflow["required_quality_gates"]
            .as_array_mut()
            .unwrap()
            .retain(|gate| gate != "end-to-end-qa");
        assert!(validate(&workflow).is_err());
    }

    #[test]
    fn unsafe_extra_fields_and_invalid_role_ids_are_rejected() {
        let mut workflow = sample();
        workflow["roles"]["business-analysis"]["command"] = json!("arbitrary shell");
        assert!(validate(&workflow).is_err());

        let mut workflow = sample();
        let first = workflow["roles"]["business-analysis"].take();
        workflow["roles"]["invalid/role-id"] = first;
        assert!(validate(&workflow).is_err());

        let mut workflow = sample();
        let first = workflow["roles"]["business-analysis"].take();
        workflow["roles"]["a".repeat(65)] = first;
        assert!(validate(&workflow).is_err());
    }

    #[test]
    fn schema_is_json_and_models_role_ids_as_unique_object_keys() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/project-workflow.schema.json")).unwrap();
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert_eq!(
            schema["$id"],
            "https://raw.githubusercontent.com/NAVANEETHVVINOD/agent-orchestrator/main/schemas/project-workflow.schema.json"
        );
        assert_eq!(schema["properties"]["roles"]["type"], "object");
        assert_eq!(validate(&sample()).unwrap()["role_count"], 14);
    }

    #[test]
    fn strict_bytes_reject_duplicate_keys_depth_and_oversize() {
        assert!(validate_bytes(br#"{"format_version":1,"format_version":1}"#).is_err());
        assert!(validate_bytes(&[b' '; 1024 * 1024 + 1]).is_err());
        let deep = format!("{}0{}", "[".repeat(100), "]".repeat(100));
        assert!(validate_bytes(deep.as_bytes()).is_err());
    }

    #[test]
    fn version_uses_json_numeric_equality_without_float_rounding() {
        for value in [
            "1",
            "1.0",
            "10e-1",
            "0.1e1",
            "1.000000000000000000000000000000000000",
        ] {
            assert!(
                numeric_one(&serde_json::from_str(value).unwrap()),
                "{value}"
            );
        }
        for value in [
            "1.000000000000000000000000000000000001",
            "1.01",
            "0.9999999999999999999999999999999999",
            "-1",
            "true",
        ] {
            let parsed: Value = serde_json::from_str(value).unwrap();
            assert!(!numeric_one(&parsed), "{value}");
        }
    }
}
