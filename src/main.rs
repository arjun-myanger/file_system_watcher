extern crate notify;

use notify::{Watcher, RecursiveMode, Result, event::{Event, EventKind}};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path_to_watch>", args[0]);
        return Ok(());
    }
    let path_to_watch = &args[1];

    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
        match res {
            Ok(event) => {
                match event.kind {
                    EventKind::Create(_) => println!("File/Folder created: {:?}", event.paths[0]),
                    _ => {}
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(Path::new(path_to_watch), RecursiveMode::Recursive)?;

    std::thread::park();  // Keeps the main thread alive indefinitely
    Ok(())
}

