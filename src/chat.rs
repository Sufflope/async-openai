use std::ops::Deref;

use async_openai::{config::AzureConfig, error::OpenAIError, types::CreateChatCompletionRequest};

use crate::{types::chat::CreateChatCompletionResponse, Client};

pub struct Chat<'c> {
    inner: async_openai::Chat<'c, AzureConfig>,
    client: &'c Client<AzureConfig>,
}

impl<'c> Deref for Chat<'c> {
    type Target = async_openai::Chat<'c, AzureConfig>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'c> Chat<'c> {
    pub fn new(client: &'c Client<AzureConfig>) -> Self {
        let inner = async_openai::Chat::new(client);
        Self { inner, client }
    }

    pub async fn create(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        if request.stream.is_some() && request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is true, use Chat::create_stream".into(),
            ));
        }
        self.client.post("/chat/completions", request).await
    }
}
