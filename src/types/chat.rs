use serde::{Deserialize, Deserializer, Serialize};

use crate::types::content_filtering::{ContentFilterResults, PromptFilterResults};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateChatCompletionResponse {
    #[serde(flatten)]
    pub vanilla: async_openai::types::CreateChatCompletionResponse,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Extra {
    pub choices: Vec<ChatChoice>,
    pub prompt_filter_results: Option<Vec<PromptFilterResults>>,
}

impl<'de> Deserialize<'de> for CreateChatCompletionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let base: serde_json::Value = serde_json::Value::deserialize(deserializer)?;

        let vanilla: async_openai::types::CreateChatCompletionResponse = serde_json::from_value(base.clone()).unwrap();
        let extra: Extra = serde_json::from_value(base).unwrap();

        Ok(CreateChatCompletionResponse{
            vanilla,
            extra
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoice {
    pub content_filter_results: Option<ContentFilterResults>,
    #[serde(flatten)]
    pub vanilla: async_openai::types::ChatChoice,
}
