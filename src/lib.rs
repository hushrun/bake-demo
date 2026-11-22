//! Finds `key=value` pairs in text and returns them as JSON.

use regex::Regex;
use serde::{Deserialize, Serialize};

/// One pair found in the text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pair {
    pub key: String,
    pub value: String,
}

/// Every `key=value` pair in `text`, in order.
pub fn pairs(text: &str) -> Vec<Pair> {
    let re = Regex::new(r"(?m)\b([A-Za-z_][A-Za-z0-9_]*)=(\S+)").expect("a valid pattern");
    re.captures_iter(text)
        .map(|c| Pair {
            key: c[1].to_string(),
            value: c[2].to_string(),
        })
        .collect()
}

/// The pairs as pretty JSON.
pub fn to_json(pairs: &[Pair]) -> serde_json::Result<String> {
    serde_json::to_string_pretty(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_pairs() {
        let found = pairs("a=1 b=two\nc=3");
        assert_eq!(found.len(), 3);
        assert_eq!(found[1].value, "two");
    }
}
