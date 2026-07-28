/// 音乐信息
pub struct MusicInfo {
    title: String,
    artist: String,
}

/// 消息
pub enum MusicInfoMessage {
    Set((String, String)),
}

impl MusicInfo {
    /// 创建
    pub fn new() -> Self {
        Self {
            title: "ShenEternity".to_string(),
            artist: "沈之永恒".to_string(),
        }
    }

    /// 更新
    pub fn update(&mut self, message: MusicInfoMessage) {
        match message {
            MusicInfoMessage::Set(info) => {
                self.title = info.0;
                self.artist = info.1;
            }
        }
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, MusicInfoMessage> {
        let title = iced::widget::text(format!("《{}》", self.title))
            .size(50)
            .color(iced::Color::from_rgb(0.9, 0.6, 0.6));

        let artist = iced::widget::text(format!("<{}>", self.artist))
            .size(40)
            .color(iced::Color::from_rgb(0.9, 0.6, 0.6));

        iced::widget::column![title, artist]
            .spacing(10)
            .align_x(iced::Alignment::Center)
            .into()
    }
}

impl Default for MusicInfo {
    /// 应付语法服务器
    fn default() -> Self {
        Self::new()
    }
}
