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

        let model = Llama::builder()
                .with_source(LlamaSource::llama_3_1_8b_chat())
                //.with_source(LlamaSource::llama_3_1_70b_chat())
                //.with_source(LlamaSource::mistral_7b_instruct_2())
                //.with_source(LlamaSource::phi_3_5_mini_4k_instruct())
                .build()
                .await
                .unwrap();
            let mut chat = Chat::builder(model)
                .with_system_prompt(["You are a helpful AI who assists with command line administration.  Please use the following history and current command line input to suggest the next command.", &history].join("\n"))
                .build();
        
            chat.add_message(command_line)
                .to_std_out()
                .await
                .unwrap();

        history.push_str("/n");
        history.push_str(command_line);
        println!("\n");

        // Now actually execute the command in the user’s default shell
        // You can also explicitly spawn "bash", "zsh", etc.
        let status = Command::new("sh")
            .arg("-c")
            .arg(command_line)
            .status();

        match status {
            Ok(s) => {
                if !s.success() {
                    eprintln!("Command exited with non-zero status: {:?}", s.code());
                }
            }
            Err(err) => {
                eprintln!("Failed to run command: {}", err);
            }
        }
    }
    
    Ok(())
}
