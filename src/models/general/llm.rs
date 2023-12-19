use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Deserialize)]
pub struct APUMessage {
    pub content: String,
}

#[derive(Deserialize)]
pub struct APIChoice {
    pub message: APUMessage,
}

#[derive(Deserialize)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>, // choices: APUMessage,
}
