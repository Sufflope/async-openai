mod chat;
pub mod types;

pub use async_openai::{config, error, Client};

use crate::{chat::Chat, config::AzureConfig};

pub trait ClientExt {
    fn chat(&self) -> Chat;
}

impl ClientExt for Client<AzureConfig> {
    fn chat(&self) -> Chat {
        Chat::new(self)
    }
}
