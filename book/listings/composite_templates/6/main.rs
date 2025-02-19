pub mod custom_button;
mod window;

use gtk::gio;
use gtk::prelude::*;
use gtk::Application;

use window::Window;

fn main() {
    // Register and include resources
    gio::resources_register_include!("composite_templates_6.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
// ANCHOR_END: build_ui
