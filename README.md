The software described is an advanced automated system that integrates artificial intelligence to process and execute system commands within a controlled environment, aiming to accomplish specific goals. It uses asynchronous programming techniques, notably async recursion, to manage and handle tasks efficiently. 

## Configuration File

Create a `config.json` file in a known directory on your host system. This file should contain the following structure:

```json
{
    "version": "1.0.0",
    "openai_api_url": "https://api.openai.com",
    "openai_api_key": "your_openai_api_key_here",
    "model": "text-davinci-003"
}
```

Replace `your_openai_api_key_here` with your actual OpenAI API key. Adjust the `version` and `model` as needed for your specific requirements.

## Running the Container

To execute a task with the `devin-clone` container, you'll use the `docker run` command with specific environment variables and volume mounts:

```bash
sudo docker run \
  -e GOAL="Your task description here" \
  -v /path/to/your/config:/app/config \
  -v /path/to/your/output:/output \
  devin-clone
```

### Parameters Explained:

- `-e GOAL="Your task description here"`: Sets the task for the AI to accomplish. Replace `Your task description here` with the specific goal you want the AI to achieve.

- `-v /path/to/your/config:/app/config`: Mounts your local `config.json` file to the container. Replace `/path/to/your/config` with the actual path to your `config.json` file.

- `-v /path/to/your/output:/output`: Mounts a local directory to the container where the results of the AI's work will be stored. Replace `/path/to/your/output` with the path to the directory where you want to store the output.

### Example:

```bash
sudo docker run \
  -e GOAL="Create a new Rust project called devintest1. Make a function that prints 'hi devin' to the console. Run the binary to confirm the output is as it should be." \
  -v ~/Documents/repos/devin-clone/config:/app/config \
  -v ~/Documents/repos/devin-clone/output:/output \
  devin-clone
```

This example sets the `GOAL` to instruct the AI to create a Rust project with specific criteria, mounts the `config.json` from `~/Documents/repos/devin-clone/config`, and specifies `~/Documents/repos/devin-clone/output` as the directory to store the output.

## Accessing the Output

After the container has finished running, the results of the AI's task will be available in the local directory mapped to `/output` in the container. You can review these results to confirm the task was completed as expected.

## Purpose and High-Level Functionality
At a high level, this software automates the execution of system commands to achieve predefined goals. It leverages AI to parse and understand tasks, then translates these into executable commands. This process involves:

Processing Requests: It begins with the system receiving a goal or a set of instructions. These instructions are then processed asynchronously. The system uses a configuration file and an AI model to generate and handle system commands.
Async Recursion: The software can call itself asynchronously. This is crucial for handling tasks that require waiting for previous commands to complete before executing the next steps, allowing for a non-blocking execution model.
Execution of Commands: Commands are executed based on the AI's analysis and understanding of the task. This execution happens within a Docker container, ensuring isolation and safety from potentially harmful operations.
Feedback Loop: The system maintains a feedback loop where the outcomes of executed commands are logged and then fed back into the AI model for further processing or to inform the user of the progress or next steps.

## AI Integration and System Commands
The integration of AI is central to interpreting tasks and generating corresponding system commands. The AI parses input messages, determines the necessary actions, and then constructs command-line instructions that are executed to achieve the goal. This process is encapsulated within an asynchronous function process_request, which recursively calls itself to handle sequential tasks and dependencies among commands.
