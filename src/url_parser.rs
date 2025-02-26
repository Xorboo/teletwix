use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

static URL_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https?://(?:www\.)?(?<domain>x\.com|twitter\.com)/[\w\-/?=&%#\.]*").unwrap());
static REPLACE_DOMAIN: &str = "fxtwitter.com";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UrlPair {
    pub original: String,
    pub updated: String,
}

pub fn extract_urls(message_text: &str) -> HashSet<UrlPair> {
    URL_PATTERN
        .captures_iter(message_text)
        .map(|caps| UrlPair {
            original: caps[0].to_string(),
            updated: caps[0].replacen(&caps["domain"], REPLACE_DOMAIN, 1),
        })
        .collect()
}
