use crate::api;
use iced::widget::button;

pub struct ListItem {
    id: u64,
    title: String,
    artist: String,
    album: String,
    image: iced::widget::image::Handle,
    duration: f64,
}

#[derive(Clone)]
pub enum ListItemMessage {
    OnPress(u64),
}

impl ListItem {
    pub fn new(data: api::data::Song, image_data: Vec<u8>) -> Self {
        let image = iced::widget::image::Handle::from_bytes(image_data);
        Self {
            id: data.id,
            title: data.title,
            artist: data.artist,
            album: data.album,
            image,
            duration: data.duration,
        }
    }

    pub fn update(&mut self, message: ListItemMessage) {
        match message {
            ListItemMessage::OnPress(_id) => {}
        }
    }

    pub fn view(&self) -> iced::Element<'static, ListItemMessage> {
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
                iced::widget::text(format!("时长:{}", time)).size(15)
            ]
            .spacing(10)
        ]
        .spacing(2);

        // 合并
        let card = iced::widget::row![album_image, music_info]
            .height(70)
            .spacing(10);

        // 让卡片可点击
        button(card)
            .width(iced::Length::Fill)
            .height(70)
            .on_press(ListItemMessage::OnPress(self.id))
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
