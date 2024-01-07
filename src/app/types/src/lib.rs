
use std::sync::{Arc, Mutex};
// use serde::{Deserialize, Serialize};

// create a struct for global AppState that has 
// a counter that can be incremented safely across threads
// as well as a Title that will not need to be updated at runtime

// #[derive(Serialize, Deserialize)]
pub struct AppState {
    pub counter: Arc<Mutex<i32>>,
    pub title: String,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            counter: Arc::new(Mutex::new(0)),
            title: String::from("Oxygen"),
        }
    }    
}

impl Clone for AppState {
    fn clone(&self) -> AppState {
        AppState {
            counter: Arc::clone(&self.counter),
            title: self.title.clone(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            counter: Arc::new(Mutex::new(0)),
            title: String::from("Oxygen"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_state_counter_is_safely_mutated_across_threads() {
        // create a new AppState
        let app_state = AppState {
            title: String::from("Oxygen"),
            ..Default::default()
        };

        // create 5 threads that increment the counter
        let mut threads = Vec::new();
        for _ in 0..5 {
            // clone the app_state
            let app_state_clone = app_state.clone();

            // spawn a thread that increments the counter
            threads.push(std::thread::spawn(move || {
                let counter = app_state_clone.counter.lock().unwrap();
                println!("thread {:?} spawned", std::thread::current().id());
                let mut counter = counter;
                println!("thread {:?} acquired lock and the value of counter is {}", std::thread::current().id(), *counter);
                *counter += 1;
                println!("thread {:?} finished and the value of counter is {}", std::thread::current().id(), *counter);
            }));
            
        }

        // join the threads
        for thread in threads {
            thread.join().unwrap();
        }
        println!("all threads joined");

        // assert that the counter is 5
        let counter = app_state.counter.lock().unwrap();
        assert_eq!(*counter, 5);
    }
}
