use teloxide::prelude::*;
use teloxide::types::ReplyParameters;

use crate::url_parser::extract_urls;

pub async fn run_bot() {
    let bot = Bot::from_env();

    let schema = Update::filter_message().branch(Message::filter_text().endpoint(process_message));

    log::info!("Starting bot...");
    Dispatcher::builder(bot, schema)
        .default_handler(|_| std::future::ready(()))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn process_message(bot: Bot, msg: Message, message_text: String) -> ResponseResult<()> {
    for url in extract_urls(&message_text) {
        bot.send_message(msg.chat.id, url)
            .reply_parameters(ReplyParameters::new(msg.id))
            .await?;
    }

    Ok(())
}
