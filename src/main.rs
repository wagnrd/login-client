use std::thread;

use crate::core::oidc;

mod core;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();
    let main_window_weak = main_window.as_weak();

    main_window.on_login_clicked(move |login_data| {
        println!("username: {} - password: {}", login_data.username, login_data.password);

        let main_window_weak = main_window_weak.clone();
        thread::spawn(move || {
            let login_result = oidc::login(login_data.username.to_string(), login_data.password.to_string());

            match login_result {
                Ok(_) => {
                    slint::invoke_from_event_loop(move || {
                        main_window_weak.unwrap().set_active_panel("launching".into());
                    }).expect("Should be executed in event loop");
                }
                Err(error) => {
                    let error_text = match error {
                        oidc::LoginError::WrongCredentials => "Wrong credentials",
                        oidc::LoginError::Generic => "Something went wrong",
                    };
                    slint::invoke_from_event_loop(move || {
                        main_window_weak.unwrap().set_error(error_text.into());
                        main_window_weak.unwrap().set_active_panel("login".into());
                    }).expect("Should be executed in event loop");
                }
            }
        });
    });

    main_window.run()
}