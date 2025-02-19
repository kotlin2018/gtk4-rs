mod todo_object;
mod todo_row;
mod window;

use gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gdk, gio};
use gtk::{CssProvider, StyleContext};

use window::Window;

fn main() {
    gio::resources_register_include!("todo_app_3.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.Todo3")
        .build();

    // Connect to signals
    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css()
    });
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/org/gtk-rs/Todo3/style.css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.present();
}
