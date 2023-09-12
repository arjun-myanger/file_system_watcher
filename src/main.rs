// Import the external "notify" library.
extern crate notify;

// Import specific items from the "notify" library and the standard library.
use std::env;
use std::path::Path;
use std::collections::HashSet;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use notify::{RecommendedWatcher, RecursiveMode, Watcher}; // Removed the unused Event import

// Import the gui module.
mod gui;

// The main function where our program starts.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the GUI
    match gui::run_gui() {
        Ok(_) => println!("GUI closed successfully."),
        Err(e) => println!("Error running GUI: {:?}", e),
    }

    // Collect the command-line arguments into a list.
    let args: Vec<String> = env::args().collect();
    
    // Check if the user provided the correct number of arguments.
    if args.len() != 2 {
        println!("Usage: {} <path_to_watch>", args[0]);
        return Ok(());
    }
    
    // Get the path provided by the user.
    let path_to_watch = &args[1];

    // Create a shared set to store recent messages.
    let recent_messages = Arc::new(Mutex::new(HashSet::new()));
    
    // Store the current time to know when to clear the set of recent messages.
    let last_clear = Arc::new(Mutex::new(Instant::now()));

    // Clone the shared set and timestamp for use inside the watcher closure.
    let messages_clone = recent_messages.clone();
    let last_clear_clone = last_clear.clone();

    // Create a file watcher with a 2-second delay.
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::watcher(tx, Duration::from_secs(2)).unwrap();

 // Used the generic watcher function

    // Start watching the specified path and all its subdirectories.
    watcher.watch(Path::new(path_to_watch), RecursiveMode::Recursive)?;

    // Keep the main thread alive indefinitely.
    std::thread::park();  
    Ok(())
}
