pub struct BackGroundImage {
    image_data: Vec<u8>,
    image_handle: iced::widget::image::Handle,
}

pub enum BackGroundImageMessage {
    Set(Vec<u8>),
}

fn blur(data: Vec<u8>) -> iced::widget::image::Handle {
    let raw_handle = iced::widget::image::Handle::from_bytes(data.clone());

    match image::ImageReader::new(std::io::Cursor::new(data)).with_guessed_format() {
        Ok(reader) => match reader.decode() {
            Ok(img) => {
                let blurred_img = img.blur(10.0);
                let width = blurred_img.width();
                let height = blurred_img.height();
                let rgba_bytes = blurred_img.to_rgba8().into_raw();

                iced::widget::image::Handle::from_rgba(width, height, rgba_bytes)
            }
            Err(_) => raw_handle.clone(),
        },
        Err(_) => raw_handle.clone(),
    }
}

impl BackGroundImage {
    pub fn new() -> Self {
        let image = include_bytes!("/home/eternity/Music/专辑图片/无法解析:未知.jpeg").to_vec();
        let image_handle = blur(image.clone());
        Self {
            image_data: image,
            image_handle,
        }
    }

    pub fn update(&mut self, message: BackGroundImageMessage) {
        match message {
            BackGroundImageMessage::Set(data) => {
                let image_handle = blur(data.clone());
                self.image_handle = image_handle;
                self.image_data = data;
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, BackGroundImageMessage> {
        iced::widget::Image::new(self.image_handle.clone())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .content_fit(iced::ContentFit::Cover)
            .into()
    }
}
impl Default for BackGroundImage {
    fn default() -> Self {
        Self::new()
    }
}
