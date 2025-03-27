use crate::config::WikiConfig;
use regex::Regex;

pub fn contains_date(input: &str) -> bool {
    let s = input.trim();

    let patterns = vec![
        r"(?i)\b\d{1,2}\s+(января|февраля|марта|апреля|мая|июня|июля|августа|сентября|октября|ноября|декабря)\s+\d{4}\b",
        r"(?i)\b\d{1,2}\s+(января|февраля|марта|апреля|мая|июня|июля|августа|сентября|октября|ноября|декабря)\b",
        r"(?i)\b(January|February|March|April|May|June|July|August|September|October|November|December)\s+\d{1,2},?\s+\d{4}\b",
        r"(?i)\b(January|February|March|April|May|June|July|August|September|October|November|December)\s+\d{1,2}\b",
        r"\b\d{4}-\d{2}-\d{2}\b",
        r"\b\d{2}\.\d{2}\.\d{4}\b",
        r"\b\d{2}/\d{2}/\d{4}\b",
    ];

    Regex::new(&format!("({})", patterns.join("|")))
        .unwrap()
        .is_match(s)
}

pub fn validate(s: &str, config: &WikiConfig) -> bool {
    let mut result = true;

    if !config.using_date && contains_date(s) {
        result = false;
    }

    result
}
