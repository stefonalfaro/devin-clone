use serde_json::json;

use crate::models::openai_request::{Parameters, Tool, ToolFunction};

pub fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "command_line".to_string(),
                description: "Runs a command in the Ubuntu command line. The response of the command will be available for you to analyze.".to_string(),
                parameters: Parameters {
                    param_type: "object".to_string(),
                    properties: json!({
                        "command": {
                            "type": "string",
                            "description": "The raw command you want to run in the cli"
                        },
                        "finished_working":{
                            "type": "string",
                            "description": "Run this when you have fully completed your goal. Be careful as you can only run this once as it stops interactions of your work to let the User know your task is fully completed. The final message you want to give to the user. Discussing your completed work and any information the User may need to know about such as problems encountered, or documentation."
                        }
                    }),
                    required: vec!["command".to_string()],
                },
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "help_or_clarification".to_string(),
                description: "Run this if you need human clarification to continue. This will pause what you are working on until the human was responsded to your question.".to_string(),
                parameters: Parameters {
                    param_type: "object".to_string(),
                    properties: json!({
                        "question": {
                            "type": "string",
                            "description": "The question you want to ask."
                        },
                    }),
                    required: vec!["question".to_string()],
                },
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "finished_working".to_string(),
                description: "Run this when you have fully completed your goal. Be careful as you can only run this once as it stops interactions of your work to let the User know your task is fully completed.".to_string(),
                parameters: Parameters {
                    param_type: "object".to_string(),
                    properties: json!({
                        "message": {
                            "type": "string",
                            "description": "The final message you want to give to the user. Discussing your completed work and any information the User may need to know about such as problems encountered, or documentation."
                        },
                    }),
                    required: vec!["message".to_string()],
                },
            },
        },
    ]
}
