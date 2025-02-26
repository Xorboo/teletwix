# teletwix

A tiny Telegram bot in Rust (using [teloxide](https://github.com/teloxide/teloxide)), that monitors Twitter/X URLs sent by users, and replies with an `fxtwitter.com` URL, allowing for proper tweet previews.

The bot is (hopefully) running on [@teletwix](https://t.me/teletwix_bot)

## Running

- Set `TELOXIDE_TOKEN` environmental variable to your token (`cp .env.example .env`)
- `cargo run`

To work in group chats, bot requires `delete message` permissions to replace the original message. If you don't need that - remove `handle_single_url()` part in `bot.rs`, or just ignore the errors.
