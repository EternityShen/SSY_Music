/// 专辑图片
pub struct Album {
    image_handle: iced::widget::image::Handle,
    title: String,
}

/// 消息
pub enum AlbumMessage {
    Set(iced::widget::image::Handle),
    SetTitle(String),
}

impl Album {
    /// 创建
    pub fn new() -> Self {
        // 一张内置的默认图片字节
        let default_image =
            include_bytes!("/home/eternity/Music/专辑图片/无法解析:未知.jpeg").to_vec();
        let image_handle = iced::widget::image::Handle::from_bytes(default_image);
        Self {
            image_handle,
            title: "Eternity".to_string(),
        }
    }

    /// 更新
    pub fn update(&mut self, message: AlbumMessage) {
        match message {
            AlbumMessage::Set(handle) => {
                self.image_handle = handle;
            }
            AlbumMessage::SetTitle(str) => {
                self.title = str;
            }
        }
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, AlbumMessage> {
        let image = iced::widget::Image::new(self.image_handle.clone())
            .content_fit(iced::ContentFit::Cover)
            .width(260)
            .height(260);

        let title = iced::widget::text(format!("《{}》", self.title))
            .size(30)
            .color(iced::Color::from_rgb(0.9, 0.6, 0.6));

        iced::widget::column![image, title]
            .spacing(10)
            .align_x(iced::Alignment::Center)
            .into()
    }
}

impl Default for Album {
    /// 应付语法服务器
    fn default() -> Self {
        Self::new()
    }
}
