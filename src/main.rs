// Import the external "notify" library.
extern crate notify;

// Import specific items from the "notify" library and the standard library.
use notify::{Watcher, RecursiveMode, Result, event::{Event, EventKind}};
use std::env;
use std::path::Path;
use std::collections::HashSet;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::fs;

// Import the gui module.
mod gui;

// The main function where our program starts.
fn main() -> Result<()> {
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

    // Create a file watcher.
    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
        match res {
            Ok(event) => {
                // Ignore changes related to the ".DS_Store" file.
                if event.paths[0].to_string_lossy().ends_with(".DS_Store") {
                    return;
                }

                // Determine the type of event (create, remove, modify) and generate a message.
                let message = match event.kind {
                    EventKind::Create(_) => format!("A new file/folder was created at: {:?}", event.paths[0]),
                    EventKind::Remove(_) => format!("A file/folder was deleted from: {:?}", event.paths[0]),
                    EventKind::Modify(_) => {
                        // Check if the file/folder still exists.
                        if fs::metadata(&event.paths[0]).is_ok() {
                            format!("A file/folder was modified at: {:?}", event.paths[0])
                        } else {
                            format!("A file/folder was deleted from: {:?}", event.paths[0])
                        }
                    },
                    _ => return,
                };

                // Check if the message was recently printed.
                let mut recent = messages_clone.lock().unwrap();
                if !recent.contains(&message) {
                    println!("{}", message);
                    recent.insert(message);
                }

                // Clear the set of recent messages every 10 seconds.
                let mut last = last_clear_clone.lock().unwrap();
                if last.elapsed() > Duration::from_secs(10) {
                    recent.clear();
                    *last = Instant::now();
                }
            },
            Err(e) => println!("An error occurred while watching: {:?}", e),
        }
    })?;

    // Start watching the specified path and all its subdirectories.
    watcher.watch(Path::new(path_to_watch), RecursiveMode::Recursive)?;

    // Keep the main thread alive indefinitely.
    std::thread::park();  
    Ok(())
}
