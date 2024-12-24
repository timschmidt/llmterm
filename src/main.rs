use std::io::{self, BufRead, Write};
use clap::{Arg, Command as ClapCommand};
use expectrl::{spawn, Eof};
use kalosm::language::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = ClapCommand::new("llmterm")
        .version("0.2.0")
        .author("Timothy Schmidt <timschmidt@gmail.com>")
        .about("Your friendly local LLM terminal companion")
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Specifies which LLM model to use")
                .value_parser([
                    "llama_3_1_8b_chat",
                    "mistral_7b_instruct_2",
                    "phi_3_5_mini_4k_instruct",
                ])
                .default_value("llama_3_1_8b_chat"),
        )
        .get_matches();

    let model_name = matches
        .get_one::<String>("model")
        .expect("Model argument must have a value");

    // Build the correct model source based on user input.
    let model_source = match model_name.as_str() {
        "llama_3_1_8b_chat" => LlamaSource::llama_3_1_8b_chat(),
        "mistral_7b_instruct_2" => LlamaSource::mistral_7b_instruct_2(),
        "phi_3_5_mini_4k_instruct" => LlamaSource::phi_3_5_mini_4k_instruct(),
        _ => {
            eprintln!("Unknown model: {}", model_name);
            std::process::exit(1);
        }
    };

    // Build the model from the chosen source.
    let model = Llama::builder()
        .with_source(model_source)
        .build()
        .await
        .unwrap();

    // Launch a *persistent* shell using `expectrl`.
    let mut shell = spawn("bash")?;

    // Change PS1 so we recognize the prompt.
    shell.send_line(r#"export PS1="llmterm> ""#)?;
    shell.expect("llmterm> ")?; // Wait until the shell prints our new prompt.

    // Now run our REPL loop in a persistent shell session.
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut history = String::new();

    loop {
        // Print a local prompt for the user
        print!("llmterm> ");
        stdout.flush()?;

        // Read one line from the user
        let mut command_line = String::new();
        if stdin.lock().read_line(&mut command_line)? == 0 {
            // EOF or read error => break
            break;
        }
        let command_line = command_line.trim();

        // If user typed 'exit' or 'quit', break out of loop
        if command_line == "exit" || command_line == "quit" {
            break;
        }

        // Record the user's command in our history
        history.push_str("\n[User Command] ");
        history.push_str(command_line);
        history.push_str("\n");

        // Send the command to our persistent shell
        shell.send_line(command_line)?;

        // Now capture everything up to the *next* prompt as this command’s output.
        // Because expect(...) returns a Result<Captures, Error>, we *match* on it:
        match shell.expect("llmterm> ") {
            Ok(captures) => {
                // `captures.before()` gives the text that appeared before the prompt.
                // from_utf8_lossy converts invalid UTF-8 to � (replacement characters)
                let mut command_output = String::from_utf8_lossy(captures.before()).to_string();

                // Remove "llmterm>" if it's in the output, or trim it if needed:
                command_output = command_output.replace("llmterm> ", "");

                // Print the output to our own stdout
                if !command_output.trim().is_empty() {
                    println!("[Shell output]{}", command_output.trim_end());
                }

                // Append to history
                if !command_output.is_empty() {
                    history.push_str("[Shell Output]\n");
                    history.push_str(&command_output);
                    history.push_str("\n");
                }
            }
            Err(e) => {
                eprintln!("Error reading shell output: {}", e);
                // If the shell still runs, we can continue; if it's fatal, break.
                continue;
            }
        }

        // Use the LLM to generate a suggestion based on the history
        let mut chat = Chat::builder(model.clone())
            .with_system_prompt(
                "You are a helpful AI who assists with command line administration. \
                 Please use the following history to suggest the next command."
            )
            .build();
        print!("[llm]\n");
        chat.add_message(["Shell history:", &history].join(" "))
            .to_std_out()
            .await
            .unwrap();

        println!();
    }

    // Optionally send an "exit" command to the shell
    shell.send_line("exit")?;
    shell.expect(Eof)?;

    Ok(())
}
