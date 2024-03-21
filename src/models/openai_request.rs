use serde::{Deserialize, Serialize};
use serde_json::Value;

//This matches the API as of Mar 20, 2024. The request object would be exactly as is. No reason for abstractions.

#[derive(Serialize, Deserialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Vec<Tool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub  tool_type: String,
    pub  function: ToolFunction,
}

#[derive(Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: Value,
    pub required: Vec<String>,
}