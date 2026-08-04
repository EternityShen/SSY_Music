// 简单到炸飞zako的代码,不需要注释

pub struct HomePage {
    title: String,
    help: String,
}

pub enum HomePageMessage {}

impl HomePage {
    pub fn new() -> Self {
        let mut user_config_dir = dirs::config_dir().ok_or("找不到系统配置目录").unwrap();
        user_config_dir.push("ssy-music");

        Self {
            title: "Welcome! Ssy-Music".to_string(),
            help: user_config_dir.to_string_lossy().to_string(),
        }
    }

    pub fn updata(&mut self, message: HomePageMessage) {
        match message {}
    }

    pub fn view(&self) -> iced::Element<'_, HomePageMessage> {
        let title = iced::widget::container(iced::widget::text(self.title.clone()).size(60))
            .align_x(iced::Alignment::Center)
            .width(iced::Length::Fill);

        let help = iced::widget::container(
            iced::widget::text(format!("配置文件和本地数据库在:{}", self.help)).size(20),
        )
        .align_x(iced::Alignment::Center)
        .width(iced::Length::Fill);

        iced::widget::column![
            iced::widget::space::vertical(),
            title,
            help,
            iced::widget::space::vertical()
        ]
        .spacing(10)
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
