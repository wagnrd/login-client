use adw::prelude::*;

mod main_window;
mod login_panel;
mod auth0_login;

fn main() -> adw::glib::ExitCode {
    let app = adw::Application::builder()
        .application_id("de.wagnrd.login_client")
        .build();
    app.connect_activate(main_window::show_window);
    app.run()
}
