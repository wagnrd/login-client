use std::convert::identity;

use relm4::{adw, Component, ComponentController, ComponentParts, ComponentSender, Controller, gtk, SimpleComponent};
use relm4::adw::prelude::*;

use crate::login_panel::LoginPanel;

pub struct App {
    login_panel: Controller<LoginPanel>,
}

#[derive(Debug)]
pub enum AppMsg {
    Increment,
    Decrement,
}

pub struct AppWidgets {}

impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = ();
    type Root = adw::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        adw::Window::builder()
            .title("Login Client")
            .default_width(300)
            .default_height(200)
            .build()
    }

    fn init(
        _args: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let login_panel = LoginPanel::builder()
            .launch(())
            .forward(sender.input_sender(), |()| { AppMsg::Decrement });


        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        vbox.append(&adw::HeaderBar::default());
        vbox.append(login_panel.widget());

        window.set_content(Some(&vbox));

        let model = App {
            login_panel
        };
        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}