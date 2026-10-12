# teletwix

A tiny Telegram bot in Rust (using [teloxide](https://github.com/teloxide/teloxide)), that monitors Twitter/X URLs sent by users, and replies with an `fxtwitter.com` URL, allowing for proper tweet previews.

The bot is (hopefully) running on [@teletwix](https://t.me/teletwix_bot)

## Running

- Set `TELOXIDE_TOKEN` environmental variable to your token (`cp .env.example .env`)
- `cargo run`

In group chats, turn off privacy mode in [@BotFather](https://t.me/BotFather) (`/setprivacy` -> `Disable`), or the bot does not see the messages. After you change this setting, remove the bot from the group and add it again.
