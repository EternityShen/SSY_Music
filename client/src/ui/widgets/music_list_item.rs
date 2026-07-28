use iced::{Length, widget::button};

/// 歌曲列表的 项
pub struct MusicListItem {
    pub id: u64,
    title: String,
    artist: String,
    duration: f64,
    album: String,
    image: iced::widget::image::Handle,
}

/// 消息
#[derive(Debug, Clone)]
pub enum MusicListItemMessage {
    OnPress(u64),
}

impl MusicListItem {
    /// 创建
    pub fn new(song_data: crate::api::data::Song, image_data: iced::widget::image::Handle) -> Self {
        Self {
            id: song_data.id,
            title: song_data.title,
            artist: song_data.artist,
            duration: song_data.duration,
            album: song_data.album,
            image: image_data,
        }
    }

    /// 更新
    pub fn update(&mut self, message: MusicListItemMessage) {
        match message {
            MusicListItemMessage::OnPress(_id) => {}
        }
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, MusicListItemMessage> {
        // 创建专辑图片
        let album_image = iced::widget::image::Image::new(self.image.clone())
            .width(50)
            .height(50);

        let time = format!(
            "{}分{}秒",
            (self.duration - (self.duration % 60.0)) / 60.0,
            (self.duration % 60.0) as u32
        );

        // 创建音乐信息
        let music_info = iced::widget::column![
            iced::widget::text(format!("歌名:{}", self.title.clone())).size(20),
            iced::widget::row![
                iced::widget::text(format!("专辑:{}", self.album.clone())).size(15),
                iced::widget::text(format!("歌手:{}", self.artist)).size(15),
                iced::widget::text(time).size(15)
            ]
            .spacing(10)
        ]
        .spacing(2);

        // 合并
        let card = iced::widget::row![album_image, music_info]
            .height(80)
            .spacing(10);

        // 让卡片可点击
        button(card)
            .width(Length::Fill)
            .height(80)
            .on_press(MusicListItemMessage::OnPress(self.id))
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
            })
            .into()
    }
}
