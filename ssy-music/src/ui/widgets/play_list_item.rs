use iced::widget::button;

pub struct PlayListItem {
    title: String,
    path: String,
}

#[derive(Debug, Clone)]
pub enum PlayListItemMessage {
    OnPress(String),
}

impl PlayListItem {
    pub fn new(title: String, path: String) -> Self {
        Self { title, path }
    }

    pub fn update(&mut self, message: PlayListItemMessage) {
        match message {
            PlayListItemMessage::OnPress(_path) => {}
        }
    }

    pub fn view(&self) -> iced::Element<'_, PlayListItemMessage> {
        let title = iced::widget::container(iced::widget::text(&self.title).size(20))
            .center_y(iced::Length::Fill);

        let card = iced::widget::row![title, iced::widget::space::horizontal()]
            .width(iced::Length::Fill)
            .height(30);

        let card_button = button(card)
            .width(iced::Length::Fill)
            .on_press(PlayListItemMessage::OnPress(self.path.clone()))
            .style(|_theme, status| match status {
                button::Status::Active => button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.2, 0.2, 0.2, 0.2,
                    ))),
                    text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: true,
                },
                button::Status::Hovered => button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.3, 0.3, 0.3, 0.3,
                    ))),
                    text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: true,
                },
                button::Status::Pressed => button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.5, 0.5, 0.5, 0.5,
                    ))),
                    text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: true,
                },
                button::Status::Disabled => button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.6, 0.6, 0.6, 0.6,
                    ))),
                    text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: true,
                },
            });

        card_button.into()
    }
}
