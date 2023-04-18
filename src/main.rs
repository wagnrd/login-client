use std::process::Command;
use std::thread;

use execute::Execute;

use crate::core::config::Config;
use crate::core::oidc;
use crate::core::oidc::{Oidc, OidcResponse};

mod core;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let config = Config::load();
    let oidc = Oidc::new(&config.oidc);
    let main_window = MainWindow::new().unwrap();

    let main_window_weak = main_window.as_weak();
    main_window.on_login_clicked(move |login_data| {
        main_window_weak.unwrap().set_active_panel("loading".into());

        let main_window = main_window_weak.clone();
        let oidc = oidc.clone();
        let app_path = config.app_path.clone();
        thread::spawn(move || {
            let login_result = oidc.login(login_data.username.to_string(), login_data.password.to_string());

            match login_result {
                Ok(response) => {
                    slint::invoke_from_event_loop(move || {
                        main_window.unwrap().set_active_panel("launching".into());
                    }).expect("Should be executed in event loop");
                    launch_app(app_path, response);
                }
                Err(error) => {
                    let error_text = match error {
                        oidc::LoginError::WrongCredentials => "Wrong credentials",
                        oidc::LoginError::Generic => "Something went wrong",
                    };
                    slint::invoke_from_event_loop(move || {
                        main_window.unwrap().set_error(error_text.into());
                        main_window.unwrap().set_active_panel("login".into());
                    }).expect("Should be executed in event loop");
                }
            }
        });
    });

    main_window.run()
}

fn launch_app(app_path: String, tokens: OidcResponse) {
    Command::new(app_path)
        .args(["--id_token", tokens.id_token.as_str()])
        .args(["--access_token", tokens.access_token.as_str()])
        .execute()
        .expect("Failed to launch application");
}