extern crate lazy_static;
use serde_json::{from_str, json};
use std::{env, fs};
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
use helpers::config::load_config;
use helpers::ai_tools::get_tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let goal: &str = "Create a new Rust project called devintest1.";
    log::warn!("Goal: {}", &goal);

    //Create the System Message and set the history of actions list
    let mut history_of_actions:Vec<String> = vec![];
    let system_message = create_system_message(&history_of_actions);
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
    let client = reqwest::Client::builder().build()?;

    //Set the Authorization Headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}",config.openai_api_key).parse()?);
    headers.insert("Content-Type", "application/json".parse()?);

    //This is the main logic loop that we need to implement recursion to. We would break the loop when the tool_call.function.name of 'finished_working' is called.
    //The idea is that we would generate a new system message includes the history, and then set the response from the last command to the user message.

    //Create the request object
    let request_data = OpenAIRequest {
        model: config.model,
        messages: messages.clone(),
        tools: get_tools()
    };

    //Send the request and get the response
    let request: reqwest::RequestBuilder = client.request(reqwest::Method::POST, &config.openai_api_url).headers(headers).json(&request_data);
    let response: reqwest::Response = request.send().await?;
    let body: String = response.text().await?;

    // Deserialize the JSON response
    let api_response: OpenAIResponse = from_str(&body)?;

    //Check if more than 2 functions, should only ever be 1.
    if api_response.choices.len() > 1 {
       log::error!("AI called two fun tions instead of one. Using the first function.");
    }

    //Handle the Function
    if let Some(first_choice) = api_response.choices.first() {
        if let Some(tool_calls) = &first_choice.message.tool_calls {
            for tool_call in tool_calls {
                log::warn!("Function Name: {}, Arguments: {}", tool_call.function.name,  tool_call.function.arguments);

                match handle_function(&tool_call) {
                    Ok(output) => {
                        log::warn!("Command {} executed successfully. Output: {}", history_of_actions.len() + 1, output);

                        //We need to add the name and arguements to the history of the system message.
                        //history_of_actions.push(format!("{}: {} {}", history_of_actions.len() + 1, tool_call.function.name, tool_call.function.arguments));

                        //Instead of the history of actions we may also use the assistant response to log the command they choose to run.
                        messages.push(Message {
                            role: "assisstant".to_string(),
                            content: goal.to_string(),
                        });

                        //The output of the command can go into the user message
                        messages.push(Message {
                            role: "user".to_string(),
                            content: format!("The command output was: {}", output.to_string()),
                        });

                        //We would now need to recursively start this process all over again. We would need to generate a new system message since the history of actions changed. We would need to pass the console output into the User message to let the AI know the console response.
                    },
                    Err(e) => {
                        log::error!("Error executing command: {}", e);
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