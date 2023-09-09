File System Watcher
A simple tool built with Rust that monitors changes in the file system. It watches directories and files for changes such as modifications, deletions, and creations. When a specified event occurs, the watcher triggers specific notifications.

Features
Watch Directories and Files: Specify which directories or files to monitor.
Real-time Notifications: Receive notifications in real-time when a watched event occurs.
Differentiate Events: The tool differentiates between various events like file creation, modification, deletion, and renaming.
User Input: Allows users to specify the directory or file they want to watch via command-line arguments.
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/arjun-myanger/file_system_watcher.git
Navigate to the project directory:

bash
Copy code
cd file_system_watcher
Build the project:

bash
Copy code
cargo build --release
Usage
Run the program with the desired path as an argument to start monitoring changes:

bash
Copy code
cargo run /path/to/your/directory
Replace /path/to/your/directory with the actual path to the directory or file you want to monitor.

Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

License
MIT





