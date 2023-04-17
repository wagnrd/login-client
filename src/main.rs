mod auth0_login;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();
    let main_window_clone = main_window.clone_strong();
    main_window.on_login_clicked(move |login_data| {
        println!("username: {} - password: {}", login_data.username, login_data.password);
        main_window_clone.set_error("Wrong credentials".into());
    });
    main_window.run()
}
