//! Reject duplicate keys, excessive depth and nonstandard JSON anywhere in a record.
use crate::Result;
use serde_json::Value;
use std::collections::HashSet;

pub fn parse(bytes: &[u8]) -> Result<Value> {
    let mut parser = Parser { bytes, position: 0 };
    parser.value(0)?;
    parser.whitespace();
    if parser.position != bytes.len() {
        return Err("Invalid JSON input".into());
    }
    serde_json::from_slice(bytes).map_err(|_| "Invalid JSON input".into())
}

struct Parser<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl Parser<'_> {
    fn whitespace(&mut self) {
        while matches!(
            self.bytes.get(self.position),
            Some(b' ' | b'\t' | b'\r' | b'\n')
        ) {
            self.position += 1;
        }
    }
    fn consume(&mut self, byte: u8) -> Result<()> {
        self.whitespace();
        if self.bytes.get(self.position) != Some(&byte) {
            return Err("Invalid JSON input".into());
        }
        self.position += 1;
        Ok(())
    }
    fn string(&mut self) -> Result<String> {
        self.whitespace();
        let start = self.position;
        self.consume(b'"')?;
        while let Some(&byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                b'"' => {
                    return serde_json::from_slice(&self.bytes[start..self.position])
                        .map_err(|_| "Invalid JSON string".into());
                }
                b'\\' => {
                    self.position += 1;
                }
                _ => {}
            }
        }
        Err("Invalid JSON string".into())
    }
    fn value(&mut self, depth: usize) -> Result<()> {
        if depth > 64 {
            return Err("JSON nesting exceeds supported depth".into());
        }
        self.whitespace();
        match self.bytes.get(self.position) {
            Some(b'{') => {
                self.position += 1;
                self.whitespace();
                if self.bytes.get(self.position) == Some(&b'}') {
                    self.position += 1;
                    return Ok(());
                }
                let mut keys = HashSet::new();
                loop {
                    let key = self.string()?;
                    if key == "$serde_json::private::Number" {
                        return Err("Reserved JSON key unsupported".into());
                    }
                    if !keys.insert(key) {
                        return Err("Duplicate JSON keys".into());
                    }
                    self.consume(b':')?;
                    self.value(depth + 1)?;
                    self.whitespace();
                    if self.bytes.get(self.position) == Some(&b'}') {
                        self.position += 1;
                        break;
                    }
                    self.consume(b',')?;
                }
            }
            Some(b'[') => {
                self.position += 1;
                self.whitespace();
                if self.bytes.get(self.position) == Some(&b']') {
                    self.position += 1;
                    return Ok(());
                }
                loop {
                    self.value(depth + 1)?;
                    self.whitespace();
                    if self.bytes.get(self.position) == Some(&b']') {
                        self.position += 1;
                        break;
                    }
                    self.consume(b',')?;
                }
            }
            Some(b'"') => {
                self.string()?;
            }
            Some(_) => {
                let start = self.position;
                while let Some(&byte) = self.bytes.get(self.position) {
                    if matches!(byte, b' ' | b'\r' | b'\n' | b'\t' | b',' | b']' | b'}') {
                        break;
                    }
                    self.position += 1;
                }
                if start == self.position {
                    return Err("Invalid JSON input".into());
                }
                let atom: Value = serde_json::from_slice(&self.bytes[start..self.position])
                    .map_err(|_| "Invalid JSON input")?;
                if matches!(atom, Value::Array(_) | Value::Object(_) | Value::String(_)) {
                    return Err("Invalid JSON atom".into());
                }
            }
            None => return Err("Invalid JSON input".into()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicates_and_nonstandard_constants_are_rejected() {
        for text in [
            r#"{"x":0,"x":1}"#,
            r#"{"a":{"x":0,"\u0078":1}}"#,
            r#"{"extra":NaN}"#,
            r#"{"extra":Infinity}"#,
            r#"{"extra":-Infinity}"#,
        ] {
            assert!(parse(text.as_bytes()).is_err());
        }
    }
    #[test]
    fn malformed_and_deep_json_are_rejected() {
        for text in [
            "{".into(),
            "[1,]".into(),
            "{}x".into(),
            format!("{}0{}", "[".repeat(100), "]".repeat(100)),
        ] {
            assert!(parse(text.as_bytes()).is_err());
        }
    }
    #[test]
    fn ordinary_nested_json_and_exact_number_are_preserved() {
        let v = parse(br#"{"x":[true,null,"\"hi"],"n":0.70000000000000000001}"#).unwrap();
        assert_eq!(v["n"].to_string(), "0.70000000000000000001");
    }
    #[test]
    fn internal_number_tag_cannot_change_json_types() {
        for input in [
            br#"{"format_version":{"$serde_json::private::Number":"1"}}"#.as_slice(),
            br#"{"proposals":[{"score":{"\u0024serde_json::private::Number":"0.9"}}]}"#.as_slice(),
        ] {
            assert!(parse(input).is_err());
        }
    }
}
