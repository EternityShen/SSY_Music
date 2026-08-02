// 简单到爆的代码,不需要注释

pub struct Info {
    title: String,
    artist: String,
}

pub enum InfoMessage {
    Set((String, String)),
}

impl Info {
    pub fn new() -> Self {
        Self {
            title: "ShenEternity".to_string(),
            artist: "Shen".to_string(),
        }
    }

    pub fn update(&mut self, message: InfoMessage) {
        match message {
            InfoMessage::Set(data) => {
                self.title = data.0;
                self.artist = data.1;
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, InfoMessage> {
        let title = iced::widget::text(self.title.clone())
            .size(50)
            .color(iced::Color::from_rgb(0.9, 0.6, 0.6))
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center);
        let artist = iced::widget::container(
            iced::widget::text(self.artist.clone())
                .size(30)
                .color(iced::Color::from_rgb(0.9, 0.6, 0.6)),
        )
        .width(iced::Length::Fill)
        .align_x(iced::alignment::Horizontal::Center);
        iced::widget::column![title, artist].spacing(10).into()
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}
