use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::types::content_filtering::{ContentFilterResults, PromptFilterResults};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateChatCompletionResponse {
    pub choices: Vec<ChatChoice>,
    pub prompt_filter_results: Option<Vec<PromptFilterResults>>,
    #[serde(flatten)]
    pub vanilla: async_openai::types::CreateChatCompletionResponse,
}

impl<'de> Deserialize<'de> for CreateChatCompletionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Field {
            Id,
            Choices,
            Created,
            Model,
            ServiceTier,
            SystemFingerprint,
            Object,
            Usage,
            PromptFilterResults,
        }

        struct V;

        impl<'de> de::Visitor<'de> for V {
            type Value = CreateChatCompletionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CreateChatCompletionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CreateChatCompletionResponse, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut id = None;
                let mut choices = None;
                let mut created = None;
                let mut model = None;
                let mut service_tier = None;
                let mut system_fingerprint = None;
                let mut object = None;
                let mut usage = None;
                let mut prompt_filter_results = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Choices => {
                            if choices.is_some() {
                                return Err(de::Error::duplicate_field("choices"));
                            }
                            choices = Some(map.next_value()?);
                        }

                        Field::Created => {
                            if created.is_some() {
                                return Err(de::Error::duplicate_field("created"));
                            }
                            created = Some(map.next_value()?);
                        }
                        Field::Model => {
                            if model.is_some() {
                                return Err(de::Error::duplicate_field("model"));
                            }
                            model = Some(map.next_value()?);
                        }
                        Field::ServiceTier => {
                            if service_tier.is_some() {
                                return Err(de::Error::duplicate_field("service_tier"));
                            }
                            service_tier = Some(map.next_value()?);
                        }
                        Field::SystemFingerprint => {
                            if system_fingerprint.is_some() {
                                return Err(de::Error::duplicate_field("system_fingerprint"));
                            }
                            system_fingerprint = Some(map.next_value()?);
                        }
                        Field::Object => {
                            if object.is_some() {
                                return Err(de::Error::duplicate_field("object"));
                            }
                            object = Some(map.next_value()?);
                        }
                        Field::Usage => {
                            if usage.is_some() {
                                return Err(de::Error::duplicate_field("usage"));
                            }
                            usage = Some(map.next_value()?);
                        }
                        Field::PromptFilterResults => {
                            if prompt_filter_results.is_some() {
                                return Err(de::Error::duplicate_field("prompt_filter_results"));
                            }
                            prompt_filter_results = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let choices: Vec<_> = choices.ok_or_else(|| de::Error::missing_field("choices"))?;
                let created = created.ok_or_else(|| de::Error::missing_field("created"))?;
                let model = model.ok_or_else(|| de::Error::missing_field("model"))?;
                let object = object.ok_or_else(|| de::Error::missing_field("object"))?;
                Ok(CreateChatCompletionResponse {
                    choices: choices.clone(),
                    prompt_filter_results,
                    vanilla: async_openai::types::CreateChatCompletionResponse {
                        id,
                        choices: choices.into_iter().map(|choice| choice.vanilla).collect(),
                        created,
                        model,
                        service_tier,
                        system_fingerprint,
                        object,
                        usage,
                    },
                })
            }
        }

        const FIELDS: &[&str] = &[
            "id",
            "choices",
            "created",
            "model",
            "service_tier",
            "system_fingerprint",
            "object",
            "usage",
            "prompt_filter_results",
        ];
        deserializer.deserialize_struct("CreateChatCompletionResponse", FIELDS, V)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoice {
    pub content_filter_results: Option<ContentFilterResults>,
    #[serde(flatten)]
    pub vanilla: async_openai::types::ChatChoice,
}
