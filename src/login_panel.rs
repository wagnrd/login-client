use adw::glib;
use adw::prelude::*;

use crate::auth0_login::{login, LoginError, OidcResponse};

pub fn build_ui() -> gtk::Stack {
    let username_text_field = adw::EntryRow::builder()
        .title("Username")
        .build();

    let password_text_field = adw::PasswordEntryRow::builder()
        .title("Password")
        .build();

    let input_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .build();
    input_container.append(&username_text_field);
    input_container.append(&password_text_field);

    let login_button = gtk::Button::builder()
        .label("Login")
        .width_request(100)
        .build();

    let login_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .build();
    login_container.append(&login_button);

    let main_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(20)
        .build();
    main_container.append(&input_container);
    main_container.append(&login_container);

    let spinner = gtk::Spinner::builder()
        .spinning(true)
        .build();

    let loading_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .build();
    loading_container.append(&spinner);

    let launch_label1 = gtk::Label::new(Some("Login success!"));
    let launch_label2 = gtk::Label::new(Some("Launching application..."));

    let launch_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(20)
        .build();
    launch_container.append(&launch_label1);
    launch_container.append(&launch_label2);

    let switch_container = gtk::Stack::builder()
        .margin_top(40)
        .margin_bottom(40)
        .margin_start(40)
        .margin_end(40)
        .build();
    switch_container.add_named(&main_container, Some("main"));
    switch_container.add_named(&loading_container, Some("loading"));
    switch_container.add_named(&launch_container, Some("launching"));
    switch_container.set_visible_child_name("main");

    let (sender, receiver) = glib::MainContext::channel::<Result<OidcResponse, LoginError>>(glib::PRIORITY_DEFAULT);
    login_button.connect_clicked(
        glib::clone!(@weak username_text_field, @weak password_text_field, @weak switch_container =>
            move |_| {
                switch_container.set_visible_child_name("loading");
                let username = username_text_field.text().to_string();
                let password = password_text_field.text().to_string();

                let sender = sender.clone();
                std::thread::spawn(move || {
                    let oidc_response = login(username, password);
                    sender.send(oidc_response).expect("Could not send through channel");
                });
            }
        )
    );
    receiver.attach(
        None,
        glib::clone!(@weak login_button, @weak switch_container, @weak main_container => @default-return Continue(false),
            move |oidcResponse| {
                match oidcResponse {
                    Ok(data) => {
                        switch_container.set_visible_child_name("launching");
                        // TODO: launch configured application with tokens
                    },
                    Err(_) => {
                        let
                        text = match oidcResponse.unwrap_err() {
                            LoginError::WrongCredentials => "Wrong credentials",
                            LoginError::Generic => "Something went wrong",
                        };
                        let label = gtk::Label::builder().build();
                        label.set_markup(format!("<span color=\"#C00\">{}</span>", text).as_str());

                        if !login_button.is_ancestor(&main_container.last_child().unwrap()) {
                            main_container.remove(&main_container.last_child().unwrap())
                        }

                        main_container.append(&label);
                        switch_container.set_visible_child_name("main");
                    }
                }

                Continue(true)
            }
        ),
    );

    switch_container
}