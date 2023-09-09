extern crate notify;

use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};
use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path_to_watch>", args[0]);
        return;
    }
    let path_to_watch = &args[1];

    let (tx, rx) = channel();

    // Create a watcher object with a delay of 1 second.
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    // Add the directory or file you want to watch.
    watcher.watch(path_to_watch, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) => println!("File created: {:?}", path),
                DebouncedEvent::Write(path) => println!("File modified: {:?}", path),
                DebouncedEvent::Remove(path) => println!("File deleted: {:?}", path),
                DebouncedEvent::Rename(old_path, new_path) => println!("File renamed from {:?} to {:?}", old_path, new_path),
                _ => println!("Other event: {:?}", event),
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
