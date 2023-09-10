use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, WindowDesc, Widget};


pub fn run_gui() -> Result<(), druid::PlatformError> {
    // Describe the main window
    let main_window = WindowDesc::new(ui_builder).title("File System Watcher");

    // Launch the application
    AppLauncher::with_window(main_window).launch(())
}

fn ui_builder() -> impl Widget<()> {
    // Create a vertical layout (column)
    let mut col = Flex::column();


    // Add a label to the column
    col.add_child(Label::new("Hello, Druid!"));

    // Add a button to the column
    let greet_button = Button::new("Greet me!")
        .on_click(|_ctx, _data, _env| {
            println!("Hello, World!");
        });
    col.add_child(greet_button);

    col
}
