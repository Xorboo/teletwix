mod bot;
mod url_parser;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    bot::run_bot().await;
}
