// Import the notify library.
extern crate notify;

// Bring in the necessary items from the notify library and standard library.
use notify::{Watcher, RecursiveMode, Result};
use std::env;
use std::path::Path;

// The main function where our program starts.
fn main() -> Result<()> {
    // Collect command-line arguments into a list.
    let args: Vec<String> = env::args().collect();

    // If the user didn't provide exactly one argument (the path to watch), show them how to use the program.
    if args.len() != 2 {
        println!("Usage: {} <path_to_watch>", args[0]);
        return Ok(());
    }

    // Get the path provided by the user.
    let path_to_watch = &args[1];

    // Create a file watcher that's best suited for the current platform.
    let mut watcher = notify::recommended_watcher(|res| {
        // When an event occurs, either print the event or an error.
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    // Start watching the specified path and all its subdirectories.
    watcher.watch(Path::new(path_to_watch), RecursiveMode::Recursive)?;

    // Indicate successful execution.
    Ok(())
}
