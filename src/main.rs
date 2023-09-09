extern crate notify;

use notify::{Watcher, RecursiveMode, Result, event::{Event, EventKind}};
use std::env;
use std::path::Path;
use std::collections::HashSet;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path_to_watch>", args[0]);
        return Ok(());
    }
    let path_to_watch = &args[1];

    // Create a shared set to store recent messages and a timestamp of the last clear operation.
    let recent_messages = Arc::new(Mutex::new(HashSet::new()));
    let last_clear = Arc::new(Mutex::new(Instant::now()));

    let messages_clone = recent_messages.clone();
    let last_clear_clone = last_clear.clone();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
        match res {
            Ok(event) => {
                // Skip events related to .DS_Store
                if event.paths[0].to_string_lossy().ends_with(".DS_Store") {
                    return;
                }

                let message = match event.kind {
                    EventKind::Create(_) => format!("A new file/folder was created at: {:?}", event.paths[0]),
                    EventKind::Remove(_) => format!("A file/folder was deleted from: {:?}", event.paths[0]),
                    EventKind::Modify(_) => {
                        // Check if the file/folder still exists to differentiate between modification and deletion
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

                // Clear the set every 10 seconds.
                let mut last = last_clear_clone.lock().unwrap();
                if last.elapsed() > Duration::from_secs(10) {
                    recent.clear();
                    *last = Instant::now();
                }
            },
            Err(e) => println!("An error occurred while watching: {:?}", e),
        }
    })?;

    watcher.watch(Path::new(path_to_watch), RecursiveMode::Recursive)?;

    std::thread::park();  // Keeps the main thread alive indefinitely
    Ok(())
}
