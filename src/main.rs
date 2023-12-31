use std::io;
use std::io::Write; // to use flush
use std::process::Command; //to run commands on the system

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
                .expect("Failed to run command");

            // Print the output
            println!("{}", String::from_utf8_lossy(&output.stdout));
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

        /*
        // look for the first word in the input
        let first_word = sentence.split_whitespace().next();
        match first_word {
            Some("run") => run_local_command(sentence),
            Some(_) => (), // move on
            None => continue, // no input so next iteration
        }
        */
        
        match input {
            "hi" => println!("Hello, oxidized world!"), // Greet the world!
            "clear" => print!("{}[2J{}[H", 27 as char, 27 as char), // Clear the terminal and move cursor to top-left  "clear" => print!("{}[2J", 27 as char), // Clear the terminal,
            "exit" => break, // Exit the loop if the input is "exit"
            "" => continue, // Ignore empty input and show the prompt again
            _ => println!("command not found: {}", input),
        }
    }
}

