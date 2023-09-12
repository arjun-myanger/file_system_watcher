// Import specific widgets from the Druid library.
use druid::widget::{Button, Flex, Label, TextBox, Controller, ControllerHost};
// Import other necessary components from Druid.
use druid::{AppLauncher, WindowDesc, Widget, Data, Lens, Selector, Target};
// Import necessary components from the notify crate.
use notify::{RecursiveMode, EventKind, RecommendedWatcher};
use std::fs;
use std::thread;
use std::time::Duration;

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
    // ... [rest of the code remains unchanged]

    // Create a button that says "Start Watching".
    let greet_button = Button::new("Start Watching")
        .on_click(move |_ctx, data: &mut AppState, _env| {
            if !data.is_watching {
                // Clone the data to move into the thread.
                let path_to_watch = data.path_to_watch.clone();
                let sink = _ctx.get_external_handle();

                // Spawn a new thread for file watching.
                thread::spawn(move || {
                    // Create the watcher with the appropriate event handler and a 2-second delay.
                    let mut watcher = watcher(move |res: Result<notify::Event, notify::Error>| {
                        // ... [rest of the code remains unchanged]
                    }, Duration::from_secs(2)).unwrap();

                    watcher.watch(&path_to_watch, RecursiveMode::Recursive).unwrap();
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
