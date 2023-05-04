use relm4::{adw, ComponentParts, ComponentSender, gtk, SimpleComponent};
use relm4::adw::prelude::*;

pub struct LoginPanel {}

#[derive(Debug)]
pub enum LoginPanelMsg {
    login,
}

pub struct AppWidgets {
    username_input: adw::EntryRow,
    password_input: adw::PasswordEntryRow,
}

impl SimpleComponent for LoginPanel {
    type Input = LoginPanelMsg;
    type Output = ();
    type Init = ();
    type Root = gtk::ListBox;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::ListBox::builder()
            .css_classes(["boxed-list", "linked"])
            .margin_top(40)
            .margin_bottom(40)
            .margin_start(40)
            .margin_end(40)
            .build()
    }

    fn init(
        _args: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let username_input = adw::EntryRow::builder()
            .title("Username")
            .build();

        let password_input = adw::PasswordEntryRow::builder()
            .title("Password")
            .build();

        let login_button = gtk::Button::builder()
            .child(&adw::ButtonContent::builder()
                .icon_name("go-next-symbolic")
                .tooltip_text("Login")
                .build())
            .css_classes(["flat"])
            .margin_top(10)
            .margin_bottom(10)
            .build();
        password_input.add_suffix(&login_button);

        root.append(&username_input);
        root.append(&password_input);

        let model = LoginPanel {};
        let widgets = AppWidgets {
            username_input,
            password_input,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {}

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
