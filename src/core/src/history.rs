use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use dirs;

pub struct History {
    commands: Vec<String>,
    current_index: Option<usize>,
    history_file_path: PathBuf,
}

impl History {
    pub fn init() -> History {
        let mut history_file_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        history_file_path.push(".oxygen_history.txt");

        let mut history = History {
            commands: Vec::new(),
            current_index: None,
            history_file_path,
        };

        history.load();
        history
    }

    fn load(&mut self) {
        if let Ok(file) = File::open(&self.history_file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(command) = line {
                    self.commands.push(command);
                }
            }
        }
    }

    pub fn add(&mut self, command: String) {
        self.commands.push(command); // Add the command to the history
        self.current_index = None; // Reset the index when a new command is added
    }

    #[allow(dead_code)]
    pub fn previous(&mut self) -> Option<&String> {
        let index = match self.current_index {
            Some(i) if i > 0 => Some(i - 1),
            Some(_) | None if !self.commands.is_empty() => Some(self.commands.len() - 1),
            _ => None,
        };

        self.current_index = index;
        index.and_then(|i| self.commands.get(i))
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> Option<&String> {
        let index = match self.current_index {
            Some(i) if i < self.commands.len() - 1 => Some(i + 1),
            _ => None,
        };

        self.current_index = index;
        index.and_then(|i| self.commands.get(i))
    }

    pub fn save(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.history_file_path)
            .expect("Failed to open history file");

        for command in &self.commands {
            writeln!(file, "{}", command).expect("Failed to write to history file");
        }
    }
}
