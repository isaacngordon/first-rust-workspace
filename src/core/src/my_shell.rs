use std::env;
use std::io;
use std::io::Write;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
// to use flush
use std::process::{Command, ExitStatus, Output}; //to run commands on the system
use std::sync::{Arc, Mutex};

use ctrlc;

use crate::history::History;

/// The main function kicking off the shell loop
pub fn run() {
    let history = Arc::new(Mutex::new(History::init()));

    // Set the Ctrl-C handler
    let history_for_ctrlc = Arc::clone(&history);
    ctrlc::set_handler(move || {
        history_for_ctrlc.lock().unwrap().save();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        print!("oxygen> "); // Print the prompt
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read user input
        let input = input.trim(); // Trim newline characters

        // Add the command to the history
        let mut history = history.lock().unwrap();
        history.add(input.to_string());
        
        // TODO: PIPE support:
        // must be peekable so we know when we are on the last command
        // source: https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
        
        match input {
            "exit" => break, // Exit the shell
            _ => (),
        }

        let output = exec(input);

        // Print the output
        println!("{}", String::from_utf8_lossy(&output.stdout));
        // Print the error message
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    // Save the history
    history.lock().unwrap().save();
}

/// Handles executing a command line
fn exec(command: &str) -> Output {
    // check if the input is a slash command
    let first_char = command.chars().next(); // Get the first character
    match first_char {
        Some('\\') => return exec_slash_command(command), // Handle slash commands
        Some(_) => (),                                    // move on
        None => {
            return Output {
                status: ExitStatus::from_raw(0), // Use a dummy exit status for non-command cases
                stdout: Vec::new(),
                stderr: Vec::new(),
            };
        } // no input so next iteration
    }

    // handle all other commands
    match command {
        "hi" => {
            return Output {
                status: ExitStatus::from_raw(0),
                stdout: "Hello, oxidized world!".as_bytes().to_vec(),
                stderr: Vec::new(),
            }
        } // Greet the world!
        "clear" => {
            print!("{}[2J{}[H", 27 as char, 27 as char);
            return Output {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            };
        } // Clear the terminal and move cursor to top-left  "clear" => print!("{}[2J", 27 as char), // Clear the terminal,
        "" => {
            return Output {
                status: ExitStatus::from_raw(0), // Use a dummy exit status for non-command cases
                stdout: Vec::new(),
                stderr: Vec::new(),
            };
        } // Ignore empty input and show the prompt again
        _ => return exec_command(command),
    }
}

/// Handles all slash commands
fn exec_slash_command(command: &str) -> Output {
    // Remove the slash from the command
    let command = command.trim_start_matches('\\');
    let first_word = command.split_whitespace().next().unwrap();

    // Check if the command is a local command
    match first_word {
        "run" => return exec_local_command(command.trim_start_matches("run ")),
        _ => {
            println!("Slash Command not found: {}", command);
            return Output {
                status: ExitStatus::from_raw(0), // Use a dummy exit status for non-command cases
                stdout: Vec::new(),
                stderr: Vec::new(),
            };
        }
    }
}

/// Handles all commands that are not slash commands
fn exec_command(input: &str) -> Output {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    match command {
        /*
         * ...the chdir command correctly changed the current directory of the process
         * created to execute it, but this process promptly terminated and had no effect
         * whatsoever on its parent shell! It was necessary to make chdir a special
         * command, executed internally within the shell. It turns out that several
         * command-like functions have the same property, for example login.
         *
         * source: https://www.bell-labs.com/usr/dmr/www/hist.html
         */
        "cd" => {
            // default to '/' as new directory if one was not provided
            let new_dir = args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(&root) {
                return Output {
                    status: ExitStatus::from_raw(1), // Non-zero exit status for error
                    stdout: Vec::new(),
                    stderr: format!("Error changing directory: {}\n", e).into_bytes(),
                };
            }
            return Output {
                status: ExitStatus::from_raw(0), // Use a dummy exit status for non-command cases
                stdout: Vec::new(),
                stderr: Vec::new(),
            };
        }
        _ => {
            return Output {
                status: ExitStatus::from_raw(0), // Use a dummy exit status for non-command cases
                stdout: Vec::new(),
                stderr: format!("{}: command not found\n", command).into_bytes(),
            };
        }
    }
}

/// Runs a command on the local system
fn exec_local_command(command: &str) -> Output {
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
            return output;
        }
        "n" | "esc" => {
            let msg = "Command execution cancelled";
            println!("{}", msg);
            return Output {
                status: ExitStatus::from_raw(0),
                stdout: msg.as_bytes().to_vec(),
                stderr: Default::default(),
            };
        }
        _ => {
            let msg = "Invalid input. Command execution cancelled";
            return Output {
                status: ExitStatus::from_raw(0),
                stdout: msg.as_bytes().to_vec(),
                stderr: Default::default(),
            };
        }
    }
}
