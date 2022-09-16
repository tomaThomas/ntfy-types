use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NtfyMsg {
    pub topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<NtfyPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<NtfyAction>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NtfyAction {
    pub action: NtfyActionType,
    pub label: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<bool>,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "u8")]
#[serde(into = "u8")]
pub enum NtfyPriority {
    Minimum,
    Low,
    Default,
    High,
    Maximum,
}

impl Default for NtfyPriority {
    fn default() -> Self {
        NtfyPriority::Default
    }
}

impl From<NtfyPriority> for u8 {
    fn from(priority: NtfyPriority) -> Self {
        match priority {
            NtfyPriority::Minimum => 1,
            NtfyPriority::Low => 2,
            NtfyPriority::Default => 3,
            NtfyPriority::High => 4,
            NtfyPriority::Maximum => 5,
        }
    }
}

impl From<u8> for NtfyPriority {
    fn from(value: u8) -> Self {
        match value {
            1 => NtfyPriority::Minimum,
            2 => NtfyPriority::Low,
            4 => NtfyPriority::High,
            5 => NtfyPriority::Maximum,
            _ => NtfyPriority::Default,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NtfyActionType {
    View,
}

impl Default for NtfyActionType {
    fn default() -> Self {
        NtfyActionType::View
    }
}

// Responses

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NtfyResponse {
    pub id: String,
    pub time: u64,
    pub event: String,
    pub topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<NtfyPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<NtfyAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<NtfyAttachment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NtfyAttachment {
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NtfyErrorResponse {
    pub code: u64,
    pub http: u16,
    pub error: String,
    pub link: String,
}
