use std::collections::HashSet;

use crate::url_parser::{UrlPair, extract_urls};
use teloxide::{
    RequestError,
    prelude::*,
    types::{MessageId, MessageKind, ReplyParameters, User},
};

pub async fn run_bot() {
    let bot = Bot::from_env();

    let schema = Update::filter_message()
        .filter_map(|update: Update| update.from().cloned())
        .branch(Message::filter_text().endpoint(process_message));

    log::info!("Starting bot...");
    Dispatcher::builder(bot, schema)
        .default_handler(|_| std::future::ready(()))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn process_message(bot: Bot, msg: Message, message_text: String, user: User) -> ResponseResult<()> {
    let urls = extract_urls(&message_text);

    if urls.len() == 1 && message_text.trim() == urls.iter().next().unwrap().original {
        handle_single_url(&bot, &msg, &urls.iter().next().unwrap().updated, &user).await?;
    } else {
        handle_multiple_urls(&bot, &msg, &urls).await?;
    }

    Ok(())
}

async fn handle_single_url(bot: &Bot, msg: &Message, updated_url: &str, user: &User) -> ResponseResult<()> {
    let MessageKind::Common(common) = &msg.kind else {
        return Ok(());
    };

    let message_text = format!(
        "[By {}] {}",
        user.mention().unwrap_or_else(|| "<unknown>".to_string()),
        updated_url
    );
    let reply_id = common.reply_to_message.as_ref().map(|msg| msg.id);
    send_message(bot, msg.chat.id, &message_text, reply_id).await?;

    bot.delete_message(msg.chat.id, msg.id)
        .await
        .map_err(|e| {
            log::info!("Failed to delete message, not enough permissions? Error: {}", e);
        })
        .ok();

    Ok(())
}

async fn handle_multiple_urls(bot: &Bot, msg: &Message, urls: &HashSet<UrlPair>) -> ResponseResult<()> {
    for url_pair in urls {
        send_message(bot, msg.chat.id, &url_pair.updated, Some(msg.id)).await?;
    }

    Ok(())
}

async fn send_message(
    bot: &Bot,
    chat_id: ChatId,
    text: &str,
    reply_to_message_id: Option<MessageId>,
) -> Result<Message, RequestError> {
    let mut send_message_task = bot.send_message(chat_id, text);
    if let Some(reply_id) = reply_to_message_id {
        send_message_task = send_message_task.reply_parameters(ReplyParameters::new(reply_id));
    }
    send_message_task.await
}
