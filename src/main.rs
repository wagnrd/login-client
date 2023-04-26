use std::process::Command;

use execute::Execute;
use iced::{Application, Settings, window};

use crate::application::App;
use crate::core::config::Config;
use crate::core::oidc::{Oidc, OidcResponse};

mod core;
mod application;

fn main() -> iced::Result {
    let config = Config::load();
    let oidc = Oidc::new(&config.oidc);

    App::run(Settings {
        window: window::Settings {
            size: (280, 170),
            resizable: false,
            ..Default::default()
        },
        ..Settings::default()
    })
}

fn launch_app(app_path: String, tokens: OidcResponse) {
    Command::new(app_path)
        .args(["--id_token", tokens.id_token.as_str()])
        .args(["--access_token", tokens.access_token.as_str()])
        .execute()
        .expect("Failed to launch application");
}