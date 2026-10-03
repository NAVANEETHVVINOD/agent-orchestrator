use crate::Result;
use rust_decimal::Decimal;
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashSet};

fn identity(value: &Value) -> Result<String> {
    let id = value.as_str().ok_or("Invalid route identity")?;
    if id.is_empty()
        || id.len() > 128
        || !id.as_bytes()[0].is_ascii_alphanumeric()
        || !id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_./:-".contains(&c))
    {
        return Err("Invalid route identity".into());
    }
    Ok(id.into())
}
fn score(value: &Value) -> Result<Decimal> {
    let number = value
        .as_number()
        .ok_or("Scores and thresholds must be numbers")?
        .to_string();
    // Exact plain decimals only: reject exponent notation and excess scale rather
    // than silently rounding a score across a routing threshold.
    if number.contains(['e', 'E']) || number.len() > 64 {
        return Err("Use exact plain decimal scores with at most 28 fractional digits".into());
    }
    let result =
        Decimal::from_str_exact(&number).map_err(|_| "Unsupported decimal score precision")?;
    if !(Decimal::ZERO..=Decimal::ONE).contains(&result) {
        return Err("Scores and thresholds must be in [0,1]".into());
    }
    Ok(result)
}
pub fn evaluate(record: &Value) -> Result<Value> {
    if !record.is_object() || record["format_version"].as_u64() != Some(1) {
        return Err("Unsupported routing record format".into());
    }
    let routes = record["available_routes"]
        .as_array()
        .ok_or("Route allowlist missing")?;
    if routes.is_empty() || routes.len() > 1000 {
        return Err("A bounded nonempty route allowlist is required".into());
    }
    let mut identities = HashSet::new();
    for value in routes {
        if !identities.insert(identity(value)?) {
            return Err("Duplicate route allowlist identity".into());
        }
    }
    let minimum = score(&record["minimum_score"])?;
    let margin = score(&record["minimum_margin"])?;
    let proposals = record["proposals"]
        .as_array()
        .ok_or("Proposal records missing")?;
    if proposals.len() != identities.len() {
        return Err("Complete route scores are required".into());
    }
    let mut observed = BTreeMap::new();
    for proposal in proposals {
        let id = identity(&proposal["route"])?;
        if !identities.contains(&id) || observed.contains_key(&id) {
            return Err("Duplicate or unavailable proposed route".into());
        }
        observed.insert(id, score(&proposal["score"])?);
    }
    let mut ranked: Vec<_> = observed.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    let (best, value) = &ranked[0];
    let reason = if *value < minimum {
        Some("below_minimum_score")
    } else if ranked.len() > 1 && (*value == ranked[1].1 || *value - ranked[1].1 < margin) {
        Some("ambiguous_or_insufficient_margin")
    } else {
        None
    };
    Ok(
        json!({"record_validation":"passed","recommendation":if reason.is_some(){"escalate"}else{"route"},
        "route":if reason.is_some(){None}else{Some(best)},"reason":reason,"execution_authorized":false,
        "capability_availability_verified":false,"model_calibration_verified":false}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(a: &str, b: &str, margin: &str) -> Value {
        crate::strict_json::parse(format!(r#"{{"format_version":1,"available_routes":["qa","research"],"minimum_score":0.6,"minimum_margin":{margin},"proposals":[{{"route":"qa","score":{a}}},{{"route":"research","score":{b}}}]}}"#).as_bytes()).unwrap()
    }
    #[test]
    fn decimal_boundary_and_just_below_are_correct() {
        for (a, b, m) in [("0.7", "0.5", "0.2"), ("0.9", "0.8", "0.1")] {
            assert_eq!(
                evaluate(&fixture(a, b, m)).unwrap()["recommendation"],
                "route"
            );
        }
        assert_eq!(
            evaluate(&fixture("0.699999999999999999", "0.5", "0.2")).unwrap()["recommendation"],
            "escalate"
        );
    }
    #[test]
    fn recommendations_never_authorize_or_certify() {
        let r = evaluate(&fixture("0.9", "0.1", "0.2")).unwrap();
        for key in [
            "execution_authorized",
            "capability_availability_verified",
            "model_calibration_verified",
        ] {
            assert_eq!(r[key], false);
        }
    }
    #[test]
    fn low_scores_ties_and_incomplete_records_escalate_or_block() {
        for (a, b) in [("0.5", "0.1"), ("0.8", "0.8"), ("0.8", "0.7")] {
            assert_eq!(
                evaluate(&fixture(a, b, "0.2")).unwrap()["recommendation"],
                "escalate"
            );
        }
        let mut r = fixture("0.9", "0.1", "0.2");
        r["proposals"][0]["route"] = json!("unknown");
        assert!(evaluate(&r).is_err());
        r = fixture("0.9", "0.1", "0.2");
        r["proposals"][1]["route"] = json!("qa");
        assert!(evaluate(&r).is_err());
        r = fixture("0.9", "0.1", "0.2");
        r["proposals"].as_array_mut().unwrap().pop();
        assert!(evaluate(&r).is_err());
    }
    #[test]
    fn invalid_numeric_types_and_missing_thresholds_block() {
        for value in [
            json!(true),
            json!(null),
            json!("0.9"),
            json!(-0.1),
            json!(1.1),
        ] {
            for field in ["minimum_score", "minimum_margin", "score"] {
                let mut r = fixture("0.9", "0.1", "0.2");
                if field == "score" {
                    r["proposals"][0][field] = value.clone();
                } else {
                    r[field] = value.clone();
                }
                assert!(evaluate(&r).is_err());
            }
        }
    }
    #[test]
    fn excess_precision_and_exponents_are_never_rounded() {
        for number in [
            "0.699999999999999999999999999999",
            "-1e-29",
            "1.000000000000000000000000000001",
            "9e-1",
        ] {
            assert!(evaluate(&fixture(number, "0.1", "0.2")).is_err());
        }
    }
}
