use std::io::{self, Write, BufRead};
use std::process::Command;
use kalosm::language::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut history = String::from("");

    loop {
        // Print a prompt
        print!("llm-shell> ");
        stdout.flush()?;

        // Read the line (command) from stdin
        let mut command_line = String::new();
        if stdin.lock().read_line(&mut command_line)? == 0 {
            // EOF or read error
            break;
        }

        // Trim whitespace/newline
        let command_line = command_line.trim();

        // If user typed 'exit' or 'quit', break.
        if command_line == "exit" || command_line == "quit" {
            break;
        }

        // Record this command in the history
        history.push_str("\n[User Command] ");
        history.push_str(command_line);
        history.push_str("\n");

        // Execute the command in a shell and capture its output
        match Command::new("sh")
            .arg("-c")
            .arg(command_line)
            .output()
        {
            Ok(output) => {
                // Convert the output from bytes to a String
                let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

                // Print to screen
                if !stdout_str.is_empty() {
                    println!("{}", stdout_str);
                }
                if !stderr_str.is_empty() {
                    eprintln!("{}", stderr_str);
                }

                // Append command output to history
                if !stdout_str.is_empty() {
                    history.push_str("[Stdout]\n");
                    history.push_str(&stdout_str);
                    history.push_str("\n");
                }
                if !stderr_str.is_empty() {
                    history.push_str("[Stderr]\n");
                    history.push_str(&stderr_str);
                    history.push_str("\n");
                }

                // Check exit status
                if !output.status.success() {
                    eprintln!("Command exited with non-zero status: {:?}", output.status.code());
                }
            }
            Err(err) => {
                eprintln!("Failed to run command: {}", err);
            }
        }
        
        // Build the model (adjust as needed for your environment)
        let model = Llama::builder()
            .with_source(LlamaSource::llama_3_1_8b_chat())
            .build()
            .await
            .unwrap();
            
        // Create the chat struct with a system prompt
        let mut chat = Chat::builder(model)
            .with_system_prompt(
                [
                    "You are a helpful AI who assists with command line administration. \
                     Please use the following history and current command line input \
                     to suggest the next command.",
                    &history
                ]
                .join("\n")
            )
            .build();

        // Add the command line as a user message
        chat.add_message(command_line)
            .to_std_out()
            .await
            .unwrap();

        // Print a blank line for spacing
        println!();
    }

    Ok(())
}

