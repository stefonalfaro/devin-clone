use serde_json::Value;
use std::process::Command;
use std::error::Error;

use crate::models::openai_response::ToolCall;

pub fn handle_function(tool_call: &ToolCall) -> Result<String, Box<dyn Error>> {
    if tool_call.function.name == "command_line" 
    {
        // Parse the arguments JSON string
        let args: Value = serde_json::from_str(&tool_call.function.arguments)?;
        if let Some(command) = args["command"].as_str() {
            // Execute the command
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()?;

            if output.status.success() {
                let output_str = String::from_utf8(output.stdout)?;
                Ok(output_str)
            } else {
                let error_str = String::from_utf8(output.stderr)?;
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_str)))
            }
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Command not found")))
        }
    }
    else if tool_call.function.name == "finished_working"
    {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "finished_working is not implemented yet.")))
    }
    else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Tool call is not 'command_line'")))
    }
}
