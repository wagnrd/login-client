use iced::{Alignment, Application, Command, Element, executor, Length, Renderer, Theme};
use iced::widget::{button, column, container, text_input};

pub struct App {}

impl Application for App {
    type Executor = executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App {},
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Login Client".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let username_input = text_input("Username", "")
            .width(200);
        let password_input = text_input("Password", "")
            .width(200)
            .password();
        let inputs = column![username_input, password_input].spacing(10);

        let login_button = button("Login");

        let content = column![inputs, login_button]
            .spacing(20)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .center_x()
            .center_y()
            .into()
    }
}