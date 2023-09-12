// Import specific items from the standard library.
use std::env;

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
    
    // Keep the main thread alive indefinitely.
    std::thread::park();  
    Ok(())
}
