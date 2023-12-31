use std::io;
use std::io::Write; // to use flush
use std::process::Command; //to run commands on the system
use std::process::Output; //to run commands on the system

/// The main function kicking off the shell loop
fn main() {
    loop {
        print!("oxygen> "); // Print the prompt
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read user input
        let input = input.trim(); // Trim newline characters

        // check if the input is a slash command
        let first_char = input.chars().next(); // Get the first character
        match first_char {
            Some('\\') => {
                handle_slash_command(input);
                continue;
            }, // Handle slash commands
            Some(_) => (), // move on
            None => continue, // no input so next iteration
        }

        // handle all other commands
        match input {
            "hi" => println!("Hello, oxidized world!"), // Greet the world!
            "clear" => print!("{}[2J{}[H", 27 as char, 27 as char), // Clear the terminal and move cursor to top-left  "clear" => print!("{}[2J", 27 as char), // Clear the terminal,
            "exit" => break, // Exit the loop if the input is "exit"
            "" => continue, // Ignore empty input and show the prompt again
            _ => handle_command(input)
        }
    }
}

/// Handles all slash commands
fn handle_slash_command(command: &str) {
    // Remove the slash from the command
    let command = command.trim_start_matches('\\');
    let first_word = command.split_whitespace().next().unwrap();

    // Check if the command is a local command
    match first_word {
        "run" => run_local_command(command.trim_start_matches("run ")),
        _ => println!("Slash Command not found: {}", command)
    }
}

/// Handles all commands that are not slash commands
fn handle_command(command: &str) {
    let mut command = command.split_whitespace();
    let command_name = command.next().unwrap();

    match command_name {
        // "run" => run_local_command(command.as_str()),
        _ => println!("Command not found: {}", command_name)
    }
}

/// Runs a command on the local system
fn run_local_command(command: &str) {
    // Confirm with user if it's okay to run the command locally
    println!("Confirm running command '{}' locally? (y/n)", command);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    // If yes, run the command. Otherwise, return.
    match input.as_str() {
        "y" | "" => {
            // Run the command
            println!(">> bash$ {}", command);
            
            //parse the command into a vector of strings
            let mut command = command.split_whitespace();
            let command_name = command.next().unwrap();
            let args: Vec<&str> = command.collect();

            // Run the command
            let output = Command::new(command_name)
                .args(args)
                .output()
                .unwrap_or_else(|error| {
                    // Print the error message
                    eprintln!("Failed to run command: {}", error);
                    
                    // Create an empty output
                    Output {
                        status: Default::default(),
                        stdout: Default::default(),
                        stderr: Default::default(),
                    }
                });

            // Print the output
            println!("{}", String::from_utf8_lossy(&output.stdout));
            // Print the error message
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        "n" | "esc" => {
            println!("Command execution cancelled");
            return;
        }
        _ => {
            println!("Invalid input. Command execution cancelled");
            return;
        }
    }
}

