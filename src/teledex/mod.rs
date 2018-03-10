mod bot;
mod result;
mod models;

pub use self::bot::DexBot;
pub use self::result::TelegramResult;
pub use self::fetcher::{DefinitionFetcher, MockDefinitionFetcher};

mod fetcher {
    pub trait DefinitionFetcher {
        fn new(name: &str) -> Self;

        fn name(&self) -> &str;

        fn get_definitions(&self, word: &str) -> Vec<&str>;
    }

    pub struct MockDefinitionFetcher {
        name: String,
    }

    impl DefinitionFetcher for MockDefinitionFetcher {
        fn new(name: &str) -> MockDefinitionFetcher {
            MockDefinitionFetcher {
                name: String::from(name),
            }
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn get_definitions(&self, word: &str) -> Vec<&str> {
            let definitions = hashmap! {
            "doggo" => vec!["good boy", "very good boy"],
            "pupper" => vec!["small good boy", "tiny good boy"],
            "woofer" => vec!["large good boy", "oversized doggo"],
            };

            match definitions.get(word) {
                Some(definitions) => definitions.to_owned(),
                None => Vec::new(),
            }
        }
    }
}
