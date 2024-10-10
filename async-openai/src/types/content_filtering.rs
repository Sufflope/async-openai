use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ContentFilteringResults<T> {
    Ok(T),
    Err(Error),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BaseResults {
    pub sexual: Option<SeverityResult>,
    pub violence: Option<SeverityResult>,
    pub hate: Option<SeverityResult>,
    pub self_harm: Option<SeverityResult>,
    pub profanity: Option<DetectedResult>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PromptResults {
    #[serde(flatten)]
    pub results: BaseResults,
    pub jailbreak: Option<DetectedResult>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChoiceResults {
    #[serde(flatten)]
    pub results: BaseResults,
    pub protected_material_text: Option<DetectedResult>,
    pub protected_material_code: Option<DetectedWithCitationResult>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DetectedResult {
    pub filtered: bool,
    pub detected: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DetectedWithCitationResult {
    #[serde(flatten)]
    pub detected_result: DetectedResult,
    pub citation: Option<Citation>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Citation {
    #[serde(rename = "lowercase")]
    pub url: Url,
    pub license: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SeverityResult {
    pub filtered: bool,
    pub severity: Option<Severity>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Safe,
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}
