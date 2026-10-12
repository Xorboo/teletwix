use std::sync::LazyLock;

use regex::Regex;

static URL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?://(?:www\.)?(?:x|twitter)\.com(?<path>/[\w\-/?=&%#\.]*)").unwrap());
static REPLACE_DOMAIN: &str = "fxtwitter.com";

pub fn extract_urls(message_text: &str) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    for caps in URL_PATTERN.captures_iter(message_text) {
        let url = format!("https://{REPLACE_DOMAIN}{}", &caps["path"]);
        if !urls.contains(&url) {
            urls.push(url);
        }
    }
    urls
}
