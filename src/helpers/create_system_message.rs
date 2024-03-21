pub fn create_system_message(history_of_actions: &Vec<String>) -> String {
    let base_message = " You are assuming the role of a CLI developer. You have been given access to the command line of a Ubunutu system and can execute any command you wish using the functions provided to you. While running commands you will be given the response so that you can analyze it to use in your decision making. You will be given a goal that you are to accomplish. Once you have accomplished the task you can call the finished_working function. You are to do all your work in the current working directory. The commands you are running are going to be executed in real life so be careful what you do. You are working in a containerized environment but be careful not to run commands that would affect the stability of the system of other running processes.
        
    ***

    
    ";
    // /History of Actions
    // Check if there are any actions in the history. If not, use "None"
    let actions_message = if history_of_actions.is_empty() {
        "".to_string()
    } else {
        history_of_actions.join("\n")
    };

    let closing_message = "
    ***

    Before responding with a command to execute, make sure you have reviewed the decision making steps to see the sub task you need to complete. Make sure to check the history of actions so you understand what you have previously done. Only run one function at a time. The User role will responsd with the output of your commands. The Assistant role will keep track of the history.
    ";

    // Concatenate the messages
    format!("{}{}{}", base_message, actions_message, closing_message)
}