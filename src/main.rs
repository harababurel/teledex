extern crate pretty_env_logger;
extern crate teledex;

use teledex::{DefinitionFetcher, DexBot, MockDefinitionFetcher};

fn main() {
    pretty_env_logger::init();

    let telegram_token = std::env::var("TELEGRAM_TOKEN")
        .expect("Must provide telegram bot token as TELEGRAM_TOKEN environment variable.");

    let mock_fetcher = MockDefinitionFetcher::new("mock");
    let mut bot = DexBot::new(&telegram_token, mock_fetcher);
    bot.run();
}
