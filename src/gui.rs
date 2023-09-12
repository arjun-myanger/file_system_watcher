// Import specific widgets from the Druid library.
use druid::widget::{Button, Flex, Label, TextBox, Controller, ControllerHost};
// Import other necessary components from Druid.
use druid::{AppLauncher, WindowDesc, Widget, Data, Lens, Selector, WidgetExt}; // Added WidgetExt for the lens method
// Import necessary components from the notify crate.
use notify::RecursiveMode;
use std::sync::mpsc; // Added for channel creation
use std::thread;
use std::time::Duration;
use druid::Target;


// Define a data structure to represent the state of our application.
#[derive(Clone, Data, Lens)]
struct AppState {
    is_watching: bool,
    message: String,
    path_to_watch: String,
}

// Selector to update the message in the AppState from the file watching thread.
const UPDATE_MESSAGE: Selector<String> = Selector::new("file-watcher.update-message");

// Custom controller to handle the UPDATE_MESSAGE command.
struct MessageController;

impl Controller<AppState, Flex<AppState>> for MessageController {
    fn event(
        &mut self,
        child: &mut Flex<AppState>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        if let druid::Event::Command(cmd) = event {
            if cmd.is(UPDATE_MESSAGE) {
                data.message = cmd.get_unchecked(UPDATE_MESSAGE).clone();
            }
        }
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
            if !data.is_watching {
                // Clone the data to move into the thread.
                let path_to_watch = data.path_to_watch.clone();
                let sink = _ctx.get_external_handle();

                // Spawn a new thread for file watching.
                thread::spawn(move || {
                    let (tx, rx) = mpsc::channel(); // Create a channel for communication
                    let mut watcher = notify::RecommendedWatcher::new(tx).unwrap();
 // Use the generic watcher function
                    watcher.watch(&path_to_watch, RecursiveMode::Recursive).unwrap();


                    // Listen for events and send them to the GUI thread
                    loop {
                        match rx.recv() {
                            Ok(event) => {
                                sink.submit_command(UPDATE_MESSAGE, Box::new(format!("{:?}", event)), Target::Auto).unwrap();
                            }
                            Err(e) => {
                                sink.submit_command(UPDATE_MESSAGE, Box::new(format!("Error: {:?}", e)), Target::Auto).unwrap();
                            }
                        }
                    }
                });

                data.message = "Started watching!".to_string();
                data.is_watching = true;
            } else {
                data.message = "Already watching!".to_string();
            }
        });
    col.add_child(greet_button);

    // Return the ControllerHost wrapping the column layout.
    ControllerHost::new(col, MessageController)
}
