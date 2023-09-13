// Import specific widgets from the Druid library.
use druid::widget::{Button, Flex, Label, TextBox, Controller, ControllerHost};
// These are the basic building blocks for creating the user interface.
// Button: Represents a clickable button.
// Flex: A flexible box layout that can contain multiple child widgets.
// Label: Displays text.
// TextBox: Allows text input.
// Controller: Provides custom behavior for widgets.
// ControllerHost: Wraps a widget and its associated controller.

// Import other necessary components from Druid.
use druid::{AppLauncher, WindowDesc, Widget, Data, Lens, Selector, WidgetExt, Target};
// AppLauncher: Starts the GUI application.
// WindowDesc: Describes a window's properties.
// Widget: Trait implemented by all UI components.
// Data: Trait for types used as data.
// Lens: Allows access to a part of data.
// Selector: Used for custom commands.
// WidgetExt: Provides additional methods for widgets.
// Target: Determines where a command should be sent.

use std::process::Command; // Allows execution of external commands.
use std::thread; // Enables multi-threading.

// Define a data structure to represent the state of our application.
#[derive(Clone, Data, Lens)]
struct AppState {
    is_watching: bool, // Indicates if the application is currently watching a directory.
    message: String, // Message to be displayed to the user.
    path_to_watch: String, // Path of the directory or file the user wants to watch.
}

// Selector to update the message in the AppState from the file watching thread.
const UPDATE_MESSAGE: Selector<String> = Selector::new("file-watcher.update-message");
// This is a unique identifier for a command that updates the message in the AppState.

// Custom controller to handle the UPDATE_MESSAGE command.
struct MessageController; // Defines a new controller type.

impl Controller<AppState, Flex<AppState>> for MessageController {
    // This function handles events for the Flex<AppState> widget.
    fn event(
        &mut self,
        child: &mut Flex<AppState>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        // Check if the event is a command.
        if let druid::Event::Command(cmd) = event {
            // If the command is UPDATE_MESSAGE, update the message in the AppState.
            if cmd.is(UPDATE_MESSAGE) {
                data.message = cmd.get_unchecked(UPDATE_MESSAGE).clone();
            }
        }
        // Forward the event to the child widget.
        child.event(ctx, event, data, env);
    }
}

// This function is responsible for launching the GUI.
pub fn run_gui() -> Result<(), druid::PlatformError> {
    // Create an initial state for our application.
    let initial_state = AppState {
        is_watching: false,
        message: "Welcome to File System Watcher!".to_string(),
        path_to_watch: "".to_string(),
    };

    // Describe the main window of the application.
    let main_window = WindowDesc::new(ui_builder).title("File System Watcher");

    // Launch the application with the main window we just described and the initial state.
    AppLauncher::with_window(main_window).launch(initial_state)
}

// This function defines the user interface of our application.
fn ui_builder() -> impl Widget<AppState> {
    // Create a vertical layout.
    let mut col = Flex::column();

    // Dynamic label that displays the message from our AppState.
    let message_label = Label::dynamic(|data: &AppState, _env| data.message.clone());
    col.add_child(message_label);

    // TextBox for user to input the path to watch.
    let path_input = TextBox::new()
        .with_placeholder("Enter path to watch")
        .lens(AppState::path_to_watch);
    col.add_child(path_input);

    // Create a button that says "Start Watching".
    let greet_button = Button::new("Start Watching")
        .on_click(move |_ctx, data: &mut AppState, _env| {
            // If not already watching, start watching.
            if !data.is_watching {
                // Clone the data to move into the thread.
                let sink = _ctx.get_external_handle();

                // Spawn a new thread for file watching.
                thread::spawn(move || {
                    // Execute the watchexec command to detect file changes.
                    let output = Command::new("watchexec")
                        .arg("--")
                        .arg("echo")
                        .arg("File changed!")
                        .output()
                        .expect("Failed to execute command");

                    // Extract the message from the command output.
                    let message = String::from_utf8_lossy(&output.stdout);
                    // Send the UPDATE_MESSAGE command with the extracted message.
                    sink.submit_command(UPDATE_MESSAGE, Box::new(message.to_string()), Target::Auto).unwrap();
                });

                // Update the AppState to reflect that we've started watching.
                data.message = "Started watching!".to_string();
                data.is_watching = true;
            } else {
                // If already watching, update the message.
                data.message = "Already watching!".to_string();
            }
        });
    col.add_child(greet_button);

    // Return the ControllerHost wrapping the column layout.
    ControllerHost::new(col, MessageController)
}
