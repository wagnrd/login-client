use slint::Model;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();
    main_window.on_on_login_clicked(|login_data| {
        println!("username: {} - passowrd: {}", login_data.username, login_data.password);
    });
    main_window.run()
}
