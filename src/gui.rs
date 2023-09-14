// Import specific widgets from the Druid library.
use druid::widget::{Button, Flex, Label, TextBox, Controller, ControllerHost};
use druid::{AppLauncher, WindowDesc, Widget, Data, Lens, Selector, WidgetExt, Target};
use std::process::{Command, Stdio};
use std::thread;
use std::io::BufRead;  // Add this line to import BufRead trait

// Define a data structure to represent the state of our application.
#[derive(Clone, Data, Lens)]
struct AppState {
    is_watching: bool,
    message: String,
    path_to_watch: String,
}

const UPDATE_MESSAGE: Selector<String> = Selector::new("file-watcher.update-message");

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

pub fn run_gui() -> Result<(), druid::PlatformError> {
    let initial_state = AppState {
        is_watching: false,
        message: "Welcome to File System Watcher!".to_string(),
        path_to_watch: "".to_string(),
    };

    let main_window = WindowDesc::new(ui_builder).title("File System Watcher");
    AppLauncher::with_window(main_window).launch(initial_state)
}

fn ui_builder() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let message_label = Label::dynamic(|data: &AppState, _env| data.message.clone());
    col.add_child(message_label);

    let path_input = TextBox::new()
    .with_placeholder("Enter path to watch")
    .lens(AppState::path_to_watch)
    .multiline(true);  // Add this line
col.add_child(path_input);

    let greet_button = Button::new("Start Watching")
        .on_click(move |_ctx, data: &mut AppState, _env| {
            if !data.is_watching {
                let sink = _ctx.get_external_handle();
                let path = data.path_to_watch.clone();

                thread::spawn(move || {
                    let mut child = Command::new("watchexec")
                        .arg("-w")
                        .arg(&path)
                        .arg("--")
                        .arg("echo")
                        .arg("File changed!")
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to execute command");

                    if let Some(output) = child.stdout.take() {
                        let reader = std::io::BufReader::new(output);
                        for line in reader.lines() {
                            let message = line.expect("Failed to read line");
                            sink.submit_command(UPDATE_MESSAGE, Box::new(message), Target::Auto).unwrap();
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

    ControllerHost::new(col, MessageController)
}
