use std::collections::HashSet;

use anyhow::Result;
use serde_json::{Map, Value};

pub mod notification_worker;
pub mod process_updates_worker;
pub mod streaming_worker;

fn diff_json(
    snapshot_1: &Map<String, Value>,
    snapshot_2: &Map<String, Value>,
) -> Result<HashSet<String>> {
    let mut field_diff = HashSet::new();

    for key in snapshot_1.keys() {
        match snapshot_2.contains_key(key) {
            true => {
                let snapshot_1_val = snapshot_1
                    .get(key)
                    .ok_or_else(|| anyhow::anyhow!("Key '{}' not found in snapshot_1", key))?;
                let snapshot_2_val = snapshot_2
                    .get(key)
                    .ok_or_else(|| anyhow::anyhow!("Key '{}' not found in snapshot_2", key))?;

                if snapshot_1_val != snapshot_2_val {
                    field_diff.insert(key.clone());
                }
            }
            false => {
                let _ = field_diff.insert(key.clone());
            }
        }
    }

    for key in snapshot_2.keys() {
        if !snapshot_1.contains_key(key) {
            field_diff.insert(key.clone());
        }
    }

    Ok(field_diff)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::json;

    use super::*;

    #[test]
    fn test_diff_json() {
        let json1 = json!({
            "name": "John",
            "age": 30,
            "address": {
                "city": "New York",
                "zip": "10001"
            },
            "phones": ["123-456-7890", "098-765-4321"]
        });

        let json2 = json!({
            "name": "John",
            "age": 31,
            "address": {
                "city": "Boston",
                "zip": "10001"
            },
            "phones": ["123-456-7890", "098-765-4321"],
            "email": "john@example.com"
        });

        let snapshot_1 = json1.as_object().unwrap();
        let snapshot_2 = json2.as_object().unwrap();

        let field_diff = diff_json(snapshot_1, snapshot_2);
        let expected_set: HashSet<String> = ["age", "address", "email"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        assert_eq!(field_diff.unwrap(), expected_set)
    }
}
