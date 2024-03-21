extern crate lazy_static;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::{from_str, Value};
use std::env;
use log::LevelFilter;
mod services {
    pub mod handle_function;
}
use services::handle_function::handle_function;
mod models {
    pub mod openai_request;
    pub mod openai_response;
}
use models::openai_request::{Message, OpenAIRequest};
use models::openai_response::OpenAIResponse;
mod helpers {
    pub mod config;
    pub mod logger;
    pub mod ai_tools;
    pub mod create_system_message;
}
use helpers::create_system_message::create_system_message;
use helpers::logger::HttpLogger;
use helpers::config::{load_config, Config};
use helpers::ai_tools::get_tools;
use async_recursion::async_recursion;
use std::process;
use crate::models::openai_request::{Function, ToolChoice};

//This custom error message is required for async recursion to ensure memory and thread safwety.
#[derive(Debug)]
struct MyError {
    message: String,
}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for MyError {}

#[async_recursion]
async fn process_request(messages: &mut Vec<Message>, client: &reqwest::Client, config: &Config) -> Result<(), MyError> {
    if messages.len() > config.max_iterations {
        log::error!("Max iterations reached.");
        return Ok(());
    }

    // Create the request object
    let request_data = OpenAIRequest {
        model: config.model.clone(),
        messages: messages.clone(),
        tools: get_tools(),
        tool_choice: ToolChoice {
            tool_type: "function".to_string(),
            function: Function {
                name: "command_line".to_string(),
            },
        },
    };

    // Setup headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Authorization", format!("Bearer {}", config.openai_api_key).parse().unwrap());

    // Send the request and get the response
    let request = client.post(&config.openai_api_url).headers(headers).json(&request_data);
    let response = request.send().await.unwrap();
    let body = response.text().await.unwrap();

    // Deserialize the JSON response
    let api_response: OpenAIResponse = from_str(body.as_str()).unwrap();

    // Handle the Function
    if api_response.choices.len() > 1 {
        log::error!("AI called two functions instead of one. Using the first function.");
    }
    if let Some(first_choice) = api_response.choices.first() {
        if let Some(tool_calls) = &first_choice.message.tool_calls {
            for tool_call in tool_calls {
                //Log the Command
                let args: Value = serde_json::from_str(&tool_call.function.arguments).unwrap();
                if let Some(command) = args["command"].as_str() {
                    log::warn!("[Command {}] {}", messages.len() - 1, command);
                } //Handle the finish working case
                else if let Some(command) = args["finished_working"].as_str() {
                    log::error!("AI has finished working and said '{}'.", command);
                    return Ok(());
                }

                match handle_function(&tool_call) {
                    Ok(output) => {
                        //Log the output
                        log::warn!("[Output {}] {}", messages.len() - 1, output);

                        //The assistant message will contain a history of the actions
                        messages.push(Message {
                            role: "assistant".to_string(),
                            content: format!("Function Name: {}, Arguments: {}", tool_call.function.name, tool_call.function.arguments),
                        });

                        // The user message will contain the response from the command.
                        messages.push(Message {
                            role: "user".to_string(),
                            content: format!("Command executed. Output from CLI: {}. Please run the next command. Check if you have completed your goal or not.", output),
                        });

                        // Recursively start this process all over again with the updated messages
                        return process_request(messages, client, config).await;
                    },
                    Err(e) => {
                        log::error!("Error executing command: {}", e);
                        return Err(MyError { message: e.to_string() });
                    }
                }
            }
        } else {
            log::error!("No tool calls for this choice.");
        }
    } else {
        log::error!("No choices available.");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    //Load the goal from the environment
    let goal: String = match env::var("GOAL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: GOAL environment variable is not set.");
            process::exit(1);
        }
    };

    //Configure the Environment and Logger
    let config_env: String = env::var("CONFIG_ENV").unwrap_or_else(|_| "default".to_string());
    if (config_env == "dev") || (config_env == "prod") { //Use BetterStack
        log::set_boxed_logger(Box::new(HttpLogger {}))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to set logger");
        log::warn!("BetterStack logging enabled.")
    }
    else { //Use a local logger
        env::set_var("RUST_LOG", "warn"); //info or warn
        env_logger::init();
        log::warn!("Local console logging enabled.");
    }
    log::warn!("Starting Devin.");

    //Load the config.json
    let config = load_config().expect("Failed to load config");

    //Set the Goal
    //let goal: &str = "Create a new Rust project called devintest1. Make a function that prints 'hi devin' to the console. Run the binary to confirm the output is as it should be.";
    log::warn!("Goal: {}", &goal);

    //Create the System Message and set the Goal
    let system_message = create_system_message(goal.to_string());
    //log::warn!("System Message: {}", &system_message);

    let mut messages: Vec<Message> =  vec![
        Message {
            role: "system".to_string(),
            content: system_message.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: goal.to_string(),
        }
    ];

    //Create the HTTP Client
    let client = reqwest::Client::builder().build().unwrap();

    //Set the Authorization Headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}",config.openai_api_key).parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    //This is the main logic loop that we need to implement recursion to. We would break the loop when the tool_call.function.name of 'finished_working' is called.
    if let Err(e) = process_request(&mut messages, &client, &config).await {
        log::error!("An error occurred: {}", e);
    }

    log::error!("Devin is shutting down.");
}