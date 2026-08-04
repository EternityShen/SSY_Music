use iced::widget::button;

pub struct PageSwitch {}

#[derive(Debug, Clone)]
pub enum PageSwitchMessage {
    Left,
    Right,
}

impl PageSwitch {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update() {}

    pub fn view(&self) -> iced::Element<'_, PageSwitchMessage> {
        let left = iced::widget::container(
            button(iced::widget::text("左").size(30))
                .height(50)
                .on_press(PageSwitchMessage::Left)
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
                }),
        )
        .center_y(iced::Length::Fill);

        let right = iced::widget::container(
            button(iced::widget::text("右").size(30))
                .height(50)
                .on_press(PageSwitchMessage::Right)
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
                }),
        )
        .center_y(iced::Length::Fill);
        iced::widget::row![left, iced::widget::space::horizontal(), right].into()
    }
}

impl Default for PageSwitch {
    fn default() -> Self {
        Self::new()
    }
}
