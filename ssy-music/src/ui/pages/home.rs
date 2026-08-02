// 简单到炸飞zako的代码,不需要注释

pub struct HomePage {
    title: String,
}

pub enum HomePageMessage {}

impl HomePage {
    pub fn new() -> Self {
        Self {
            title: "Welcome! Syy-Music".to_string(),
        }
    }

    pub fn updata(&mut self, message: HomePageMessage) {
        match message {}
    }

    pub fn view(&self) -> iced::Element<'_, HomePageMessage> {
        let title = iced::widget::container(iced::widget::text(self.title.clone()).size(60))
            .align_x(iced::Alignment::Center)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill);

        iced::widget::column![title]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

impl Default for HomePage {
    fn default() -> Self {
        Self::new()
    }
}
