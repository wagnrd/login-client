use adw::prelude::*;

use crate::login_panel;

pub fn show_window(app: &adw::Application) {
    let container_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    container_box.append(&adw::HeaderBar::new());
    container_box.append(&login_panel::build_ui());

    adw::ApplicationWindow::builder()
        .application(app)
        .title("Login Client")
        .content(&container_box)
        .build()
        .present();
}