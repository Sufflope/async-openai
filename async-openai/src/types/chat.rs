use std::{collections::HashMap, pin::Pin};

use derive_builder::Builder;
use futures::Stream;
use serde::{Deserialize, Serialize};

use super::{ChoiceResults, ContentFilteringResults};
use crate::error::OpenAIError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Prompt {
    String(String),
    StringArray(Vec<String>),
    // Minimum value is 0, maximum value is 50256 (inclusive).
    IntegerArray(Vec<u16>),
    ArrayOfIntegerArray(Vec<Vec<u16>>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Stop {
    String(String),           // nullable: true
    StringArray(Vec<String>), // minItems: 1; maxItems: 4
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Logprobs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<Option<f32>>, // Option is to account for null value in the list
    pub top_logprobs: Vec<serde_json::Value>,
    pub text_offset: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompletionFinishReason {
    Stop,
    Length,
    ContentFilter,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Choice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<Logprobs>,
    pub finish_reason: Option<CompletionFinishReason>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChatCompletionFunctionCall {
    /// The model does not call a function, and responds to the end-user.
    #[serde(rename = "none")]
    None,
    /// The model can pick between an end-user or calling a function.
    #[serde(rename = "auto")]
    Auto,

    // In spec this is ChatCompletionFunctionCallOption
    // based on feedback from @m1guelpf in https://github.com/64bit/async-openai/pull/118
    // it is diverged from the spec
    /// Forces the model to call the specified function.
    #[serde(untagged)]
    Function { name: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
    Tool,
    Function,
}

/// The name and arguments of a function that should be called, as generated by the model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FunctionCall {
    /// The name of the function to call.
    pub name: String,
    /// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
    pub arguments: String,
}

/// Usage statistics for the completion request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CompletionUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Number of tokens in the generated completion.
    pub completion_tokens: u32,
    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestSystemMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestSystemMessage {
    /// The contents of the system message.
    pub content: ChatCompletionRequestSystemMessageContent,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartTextArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessageContentPartText {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartRefusal {
    /// The refusal message generated by the model.
    pub refusal: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    #[default]
    Auto,
    Low,
    High,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ImageUrlArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ImageUrl {
    /// Either a URL of the image or the base64 encoded image data.
    pub url: String,
    /// Specifies the detail level of the image. Learn more in the [Vision guide](https://platform.openai.com/docs/guides/vision/low-or-high-fidelity-image-understanding).
    pub detail: Option<ImageDetail>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartImageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessageContentPartImage {
    pub image_url: ImageUrl,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestUserMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    ImageUrl(ChatCompletionRequestMessageContentPartImage),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestSystemMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Refusal(ChatCompletionRequestMessageContentPartRefusal),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestToolMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    /// The text contents of the system message.
    Text(String),
    /// An array of content parts with a defined type. For system messages, only type `text` is supported.
    Array(Vec<ChatCompletionRequestSystemMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    /// The text contents of the message.
    Text(String),
    /// An array of content parts with a defined type, each can be of type `text` or `image_url` when passing in images. You can pass multiple images by adding multiple `image_url` content parts. Image input is only supported when using the `gpt-4o` model.
    Array(Vec<ChatCompletionRequestUserMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    /// The text contents of the message.
    Text(String),
    /// An array of content parts with a defined type. Can be one or more of type `text`, or exactly one of type `refusal`.
    Array(Vec<ChatCompletionRequestAssistantMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
    /// The text contents of the tool message.
    Text(String),
    /// An array of content parts with a defined type. For tool messages, only type `text` is supported.
    Array(Vec<ChatCompletionRequestToolMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestUserMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestUserMessage {
    /// The contents of the user message.
    pub content: ChatCompletionRequestUserMessageContent,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestAssistantMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestAssistantMessage {
    /// The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ChatCompletionRequestAssistantMessageContent>,
    /// The refusal message by the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
    /// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

/// Tool message
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestToolMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestToolMessage {
    /// The contents of the tool message.
    pub content: ChatCompletionRequestToolMessageContent,
    pub tool_call_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestFunctionMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestFunctionMessage {
    /// The return value from the function call, to return to the model.
    pub content: Option<String>,
    /// The name of the function to call.
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "role")]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionRequestMessage {
    System(ChatCompletionRequestSystemMessage),
    User(ChatCompletionRequestUserMessage),
    Assistant(ChatCompletionRequestAssistantMessage),
    Tool(ChatCompletionRequestToolMessage),
    Function(ChatCompletionRequestFunctionMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: ChatCompletionToolType,
    /// The function that the model called.
    pub function: FunctionCall,
}

/// A chat completion message generated by the model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessage {
    /// The contents of the message.
    pub content: Option<String>,
    /// The refusal message generated by the model.
    pub refusal: Option<String>,
    /// The tool calls generated by the model, such as function calls.
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,

    /// The role of the author of this message.
    pub role: Role,

    /// Deprecated and replaced by `tool_calls`.
    /// The name and arguments of a function that should be called, as generated by the model.
    #[deprecated]
    pub function_call: Option<FunctionCall>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "ChatCompletionFunctionsArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
#[deprecated]
pub struct ChatCompletionFunctions {
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters the functions accepts, described as a JSON Schema object. See the [guide](https://platform.openai.com/docs/guides/text-generation/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.
    ///
    /// Omitting `parameters` defines a function with an empty parameter list.
    pub parameters: serde_json::Value,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "FunctionObjectArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct FunctionObject {
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters the functions accepts, described as a JSON Schema object. See the [guide](https://platform.openai.com/docs/guides/text-generation/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.
    ///
    /// Omitting `parameters` defines a function with an empty parameter list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,

    /// Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](https://platform.openai.com/docs/guides/function-calling).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    /// The type of response format being defined: `text`
    Text,
    /// The type of response format being defined: `json_object`
    JsonObject,
    /// The type of response format being defined: `json_schema`
    JsonSchema {
        json_schema: ResponseFormatJsonSchema,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ResponseFormatJsonSchema {
    /// A description of what the response format is for, used by the model to determine how to respond in the format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the response format. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length
    pub name: String,
    /// The schema for the response format, described as a JSON Schema object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<serde_json::Value>,
    /// Whether to enable strict schema adherence when generating the output. If set to true, the model will always follow the exact schema defined in the `schema` field. Only a subset of JSON Schema is supported when `strict` is `true`. To learn more, read the [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolType {
    #[default]
    Function,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[builder(name = "ChatCompletionToolArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionTool {
    #[builder(default = "ChatCompletionToolType::Function")]
    pub r#type: ChatCompletionToolType,
    pub function: FunctionObject,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct FunctionName {
    /// The name of the function to call.
    pub name: String,
}

/// Specifies a tool the model should use. Use to force the model to call a specific function.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ChatCompletionNamedToolChoice {
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: ChatCompletionToolType,

    pub function: FunctionName,
}

/// Controls which (if any) tool is called by the model.
/// `none` means the model will not call any tool and instead generates a message.
/// `auto` means the model can pick between generating a message or calling one or more tools.
/// `required` means the model must call one or more tools.
/// Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.
///
/// `none` is the default when no tools are present. `auto` is the default if tools are present.present.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOption {
    #[default]
    None,
    Auto,
    Required,
    #[serde(untagged)]
    Named(ChatCompletionNamedToolChoice),
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTierResponse {
    Scale,
    Default,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[builder(name = "CreateChatCompletionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateChatCompletionRequest {
    /// A list of messages comprising the conversation so far. [Example Python code](https://cookbook.openai.com/examples/how_to_format_inputs_to_chatgpt_models).
    pub messages: Vec<ChatCompletionRequestMessage>, // min: 1

    /// ID of the model to use.
    /// See the [model endpoint compatibility](https://platform.openai.com/docs/models/model-endpoint-compatibility) table for details on which models work with the Chat API.
    pub model: String,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>, // min: -2.0, max: 2.0, default: 0

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>, // default: null

    /// Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the `content` of `message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability. `logprobs` must be set to `true` if this parameter is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,

    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) that can be generated in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to minimize costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1, max: 128, default: 1

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>, // min: -2.0, max: 2.0, default 0

    /// An object specifying the format that the model must output. Compatible with [GPT-4o](https://platform.openai.com/docs/models/gpt-4o), [GPT-4o mini](https://platform.openai.com/docs/models/gpt-4o-mini), [GPT-4 Turbo](https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo) and all GPT-3.5 Turbo models newer than `gpt-3.5-turbo-1106`.
    ///
    /// Setting to `{ "type": "json_schema", "json_schema": {...} }` enables Structured Outputs which guarantees the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    ///
    /// Setting to `{ "type": "json_object" }` enables JSON mode, which guarantees the message the model generates is valid JSON.
    ///
    /// **Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly "stuck" request. Also note that the message content may be partially cut off if `finish_reason="length"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    ///  This feature is in Beta.
    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests
    /// with the same `seed` and parameters should return the same result.
    /// Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Specifies the latency tier to use for processing the request. This parameter is relevant for customers subscribed to the scale tier service:
    /// - If set to 'auto', the system will utilize scale tier credits until they are exhausted.
    /// - If set to 'default', the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.
    /// - When not set, the default behavior is 'auto'.
    ///
    /// When this parameter is set, the response body will include the `service_tier` utilized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    /// Tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format)
    /// as they become available, with the stream terminated by a `data: [DONE]` message. [Example Python code](https://cookbook.openai.com/examples/how_to_stream_completions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ChatCompletionStreamOptions>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // min: 0, max: 2, default: 1,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    ///  We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // min: 0, max: 1, default: 1

    /// A list of tools the model may call. Currently, only functions are supported as a tool.
    /// Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoiceOption>,

    /// Whether to enable [parallel function calling](https://platform.openai.com/docs/guides/function-calling/parallel-function-calling) during tool use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Deprecated in favor of `tool_choice`.
    ///
    /// Controls which (if any) function is called by the model.
    /// `none` means the model will not call a function and instead generates a message.
    /// `auto` means the model can pick between generating a message or calling a function.
    /// Specifying a particular function via `{"name": "my_function"}` forces the model to call that function.
    ///
    /// `none` is the default when no functions are present. `auto` is the default if functions are present.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCall>,

    /// Deprecated in favor of `tools`.
    ///
    /// A list of functions the model may generate JSON inputs for.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<ChatCompletionFunctions>>,
}

/// Options for streaming response. Only set this when you set `stream: true`.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ChatCompletionStreamOptions {
    /// If set, an additional chunk will be streamed before the `data: [DONE]` message. The `usage` field on this chunk shows the token usage statistics for the entire request, and the `choices` field will always be an empty array. All other chunks will also include a `usage` field, but with a null value.
    pub include_usage: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    FunctionCall,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TopLogprobs {
    /// The token.
    pub token: String,
    /// The log probability of this token.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionTokenLogprob {
    /// The token.
    pub token: String,
    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
    ///  List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned.
    pub top_logprobs: Vec<TopLogprobs>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoiceLogprobs {
    /// A list of message content tokens with log probability information.
    pub content: Option<Vec<ChatCompletionTokenLogprob>>,
    pub refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoice {
    /// The index of the choice in the list of choices.
    pub index: u32,
    pub message: ChatCompletionResponseMessage,
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
    /// `length` if the maximum number of tokens specified in the request was reached,
    /// `content_filter` if content was omitted due to a flag from our content filters,
    /// `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.
    pub finish_reason: Option<FinishReason>,
    /// Log probability information for the choice.
    pub logprobs: Option<ChatChoiceLogprobs>,
    pub content_filter_results: Option<ContentFilteringResults<ChoiceResults>>,
}

/// Represents a chat completion response returned by model, based on the provided input.
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateChatCompletionResponse {
    /// A unique identifier for the chat completion.
    pub id: String,
    /// A list of chat completion choices. Can be more than one if `n` is greater than 1.
    pub choices: Vec<ChatChoice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: u32,
    /// The model used for the chat completion.
    pub model: String,
    /// The service tier used for processing the request. This field is only included if the `service_tier` parameter is specified in the request.
    pub service_tier: Option<ServiceTierResponse>,
    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`.
    pub object: String,
    pub usage: Option<CompletionUsage>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
pub type ChatCompletionResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, OpenAIError>> + Send>>;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FunctionCallStream {
    /// The name of the function to call.
    pub name: Option<String>,
    /// The arguments to call the function with, as generated by the model in JSON format.
    /// Note that the model does not always generate valid JSON, and may hallucinate
    /// parameters not defined by your function schema. Validate the arguments in your
    /// code before calling your function.
    pub arguments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCallChunk {
    pub index: i32,
    /// The ID of the tool call.
    pub id: Option<String>,
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: Option<ChatCompletionToolType>,
    pub function: Option<FunctionCallStream>,
}

/// A chat completion delta generated by streamed model responses.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionStreamResponseDelta {
    /// The contents of the chunk message.
    pub content: Option<String>,
    /// The name and arguments of a function that should be called, as generated by the model.
    #[deprecated]
    pub function_call: Option<FunctionCallStream>,

    pub tool_calls: Option<Vec<ChatCompletionMessageToolCallChunk>>,
    /// The role of the author of this message.
    pub role: Option<Role>,
    /// The refusal message generated by the model.
    pub refusal: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoiceStream {
    /// The index of the choice in the list of choices.
    pub index: u32,
    pub delta: ChatCompletionStreamResponseDelta,
    pub finish_reason: Option<FinishReason>,
    /// Log probability information for the choice.
    pub logprobs: Option<ChatChoiceLogprobs>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
/// Represents a streamed chunk of a chat completion response returned by model, based on the provided input.
pub struct CreateChatCompletionStreamResponse {
    /// A unique identifier for the chat completion. Each chunk has the same ID.
    pub id: String,
    /// A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the last chunk if you set `stream_options: {"include_usage": true}`.
    pub choices: Vec<ChatChoiceStream>,

    /// The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    pub created: u32,
    /// The model to generate the completion.
    pub model: String,
    /// The service tier used for processing the request. This field is only included if the `service_tier` parameter is specified in the request.
    pub service_tier: Option<ServiceTierResponse>,
    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: Option<String>,
    /// The object type, which is always `chat.completion.chunk`.
    pub object: String,

    /// An optional field that will only be present when you set `stream_options: {"include_usage": true}` in your request.
    /// When present, it contains a null value except for the last chunk which contains the token usage statistics for the entire request.
    pub usage: Option<CompletionUsage>,
}
