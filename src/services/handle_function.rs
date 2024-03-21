use serde_json::Value;
use std::process::Command;
use crate::models::openai_response::ToolCall;
use crate::MyError;

pub fn handle_function(tool_call: &ToolCall) -> Result<String, MyError> {
    if tool_call.function.name == "command_line" 
    {
        // Parse the arguments JSON string
        let args: Value = serde_json::from_str(&tool_call.function.arguments).unwrap();
        if let Some(command) = args["command"].as_str() {
            // Execute the command
            let output = Command::new("sh")
                .current_dir("./output")
                .arg("-c")
                .arg(command)
                .output().unwrap();

            if output.status.success() {
                let output_str = String::from_utf8(output.stdout).unwrap();
                Ok(output_str)
            } else {
                let error_str = String::from_utf8(output.stderr).unwrap();
                Ok(error_str) //Make sure to return the Err as an Ok so the AI can read the response. This isn't a real error.
            }
        } else {
            return Err(MyError { message: "Command not found".to_string() });
        }
    }
    else if tool_call.function.name == "finished_working"
    {
        return Err(MyError { message: "finished_working is not implemented yet.".to_string() });
    }
    else {
        return Err(MyError { message: "Tool call is not 'command_line'".to_string() });
    }
}
