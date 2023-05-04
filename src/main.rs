use std::process::Command;

use execute::Execute;
use relm4::RelmApp;

use crate::application::App;
use crate::core::config::Config;
use crate::core::oidc::{Oidc, OidcResponse};

mod core;
mod application;
mod login_panel;

fn main() {
    let config = Config::load();
    let oidc = Oidc::new(&config.oidc);

    let app = RelmApp::new("de.wagnrd.login-client");
    app.run::<App>(());
}

fn launch(app_path: String, tokens: OidcResponse) {
    Command::new(app_path)
        .args(["--id_token", tokens.id_token.as_str()])
        .args(["--access_token", tokens.access_token.as_str()])
        .execute()
        .expect("Failed to launch application");
}