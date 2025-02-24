use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use teloxide::{prelude::*, types::ReplyParameters};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

static URL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"https?://(?:www\.)?(?<domain>x\.com|twitter\.com)/[\w\-/?=&%#\.]*").unwrap()
});
static REPLACE_DOMAIN: &str = "fxtwitter.com";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();

    let schema = Update::filter_message().branch(Message::filter_text().endpoint(process_message));
    Dispatcher::builder(bot, schema).build().dispatch().await;
}

async fn process_message(bot: Bot, msg: Message, message_text: String) -> Result<(), Error> {
    let urls = fix_twix_urls(message_text);
    for url in urls {
        bot.send_message(msg.chat.id, &url)
            .reply_parameters(ReplyParameters::new(msg.id))
            .await?;
    }

    Ok(())
}

fn fix_twix_urls(message_text: String) -> HashSet<String> {
    URL_PATTERN
        .captures_iter(&message_text)
        .map(|caps| {
            let url = &caps[0];
            let domain = &caps["domain"];
            url.replacen(domain, REPLACE_DOMAIN, 1)
        })
        .collect()
}
