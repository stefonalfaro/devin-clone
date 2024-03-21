use serde_json::json;

use crate::models::openai_request::{Parameters, Tool, ToolFunction};

pub fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "command_line".to_string(),
                description: "Runs a command in the Ubuntu command line. The response of the command will be available for you to analyze. Remember that when you have finished your goal, to use the finished_working flag.".to_string(),
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
        }
    ]
}
