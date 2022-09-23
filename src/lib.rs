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
    pub actions: Option<Vec<NtfyAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl NtfyMsg {
    pub fn new(topic: &str) -> NtfyMsg {
        NtfyMsg {
            topic: String::from(topic),
            ..Default::default()
        }
    }

    pub fn builder(topic: &str) -> NtfyMsgBuilder {
        NtfyMsgBuilder::new(topic)
    }
}

pub struct NtfyMsgBuilder {
    msg: NtfyMsg,
}

impl NtfyMsgBuilder {
    pub fn new(topic: &str) -> NtfyMsgBuilder {
        NtfyMsgBuilder {
            msg: NtfyMsg::new(topic),
        }
    }

    pub fn topic(mut self, topic: &str) -> NtfyMsgBuilder {
        self.msg.topic = String::from(topic);
        self
    }

    pub fn message(mut self, message: &str) -> NtfyMsgBuilder {
        self.msg.message = Some(String::from(message));
        self
    }

    pub fn title(mut self, title: &str) -> NtfyMsgBuilder {
        self.msg.title = Some(String::from(title));
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> NtfyMsgBuilder {
        self.msg.tags = Some(tags);
        self
    }

    pub fn add_tag(mut self, tag: &str) -> NtfyMsgBuilder {
        if self.msg.tags.is_none() {
            self.msg.tags = Some(vec![String::from(tag)]);
        } else {
            self.msg.tags.as_mut().unwrap().push(String::from(tag));
        }
        self
    }

    pub fn priority(mut self, priority: NtfyPriority) -> NtfyMsgBuilder {
        self.msg.priority = Some(priority);
        self
    }

    pub fn attach(mut self, attach: &str) -> NtfyMsgBuilder {
        self.msg.attach = Some(String::from(attach));
        self
    }

    pub fn filename(mut self, filename: &str) -> NtfyMsgBuilder {
        self.msg.filename = Some(String::from(filename));
        self
    }

    pub fn click(mut self, click: &str) -> NtfyMsgBuilder {
        self.msg.click = Some(String::from(click));
        self
    }

    pub fn actions(mut self, actions: Vec<NtfyAction>) -> NtfyMsgBuilder {
        self.msg.actions = Some(actions);
        self
    }

    pub fn add_action(mut self, action: NtfyAction) -> NtfyMsgBuilder {
        if self.msg.actions.is_none() {
            self.msg.actions = Some(vec![action]);
        } else {
            self.msg.actions.as_mut().unwrap().push(action);
        }
        self
    }

    pub fn delay(mut self, delay: &str) -> NtfyMsgBuilder {
        self.msg.delay = Some(String::from(delay));
        self
    }

    pub fn email(mut self, email: &str) -> NtfyMsgBuilder {
        self.msg.email = Some(String::from(email));
        self
    }

    pub fn build(self) -> NtfyMsg {
        self.msg
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NtfyAction {
    pub action: NtfyActionType,
    pub label: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<bool>,
}

impl NtfyAction {
    pub fn new(label: String, url: String) -> NtfyAction {
        NtfyAction {
            action: NtfyActionType::View,
            label,
            url,
            clear: None,
        }
    }
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
