use std::io;
use std::io::Write; // to use flush

fn main() {
    loop {
        print!("oxygen> "); // Print the prompt
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read user input
        let input = input.trim(); // Trim newline characters

        match input {
            "breathe" => println!("Hmmmmmmm aaaahhhhh"),
            "exit" => break, // Exit the loop if the input is "exit"
            "" => continue, // Ignore empty input and show the prompt again
            _ => println!("command not found: {}", input),
        }
    }
}

