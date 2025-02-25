use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use teloxide::{
    prelude::*,
    types::{MessageKind, ReplyParameters},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

static URL_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https?://(?:www\.)?(?<domain>x\.com|twitter\.com)/[\w\-/?=&%#\.]*").unwrap());
static REPLACE_DOMAIN: &str = "fxtwitter.com";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct UrlPair {
    original: String,
    updated: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();
    let schema = Update::filter_message().branch(Message::filter_text().endpoint(process_message));
    Dispatcher::builder(bot, schema).build().dispatch().await;
}

async fn process_message(bot: Bot, msg: Message, message_text: String) -> Result<(), Error> {
    if msg.from.map_or(true, |from| from.is_bot) {
        return Ok(());
    }

    let urls = extract_urls(&message_text);

    if urls.len() == 1 && message_text.trim() == urls.iter().next().unwrap().original {
        let single_url = &urls.iter().next().unwrap().updated;

        if let MessageKind::Common(common) = &msg.kind {
            if let Some(original_msg) = &common.reply_to_message {
                bot.send_message(msg.chat.id, single_url)
                    .reply_parameters(ReplyParameters::new(original_msg.id))
                    .await?;
            } else {
                bot.send_message(msg.chat.id, single_url).await?;
            }

            bot.delete_message(msg.chat.id, msg.id).await?;
        }
    } else {
        for url_pair in urls {
            bot.send_message(msg.chat.id, &url_pair.updated)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
        }
    }

    Ok(())
}

fn extract_urls(message_text: &str) -> HashSet<UrlPair> {
    URL_PATTERN
        .captures_iter(message_text)
        .map(|caps| UrlPair {
            original: caps[0].to_string(),
            updated: caps[0].replacen(&caps["domain"], REPLACE_DOMAIN, 1),
        })
        .collect()
}
