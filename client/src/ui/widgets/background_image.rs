use iced::Element;
use iced::widget::image::Handle;
use image::ImageReader;
use std::io::Cursor;

pub struct BackGroundImage {
    pub raw_bytes: Vec<u8>,     // 存储原始图片字节
    pub blurred_handle: Handle, // 模糊后的 iced 图片句柄
}

/// 消息
pub enum BackGroundImageMessage {
    Set(Vec<u8>), // 接收字节数组
}

impl Default for BackGroundImage {
    /// 默认构造
    fn default() -> Self {
        // 包含一张内置的默认图片字节（比如用 include_bytes! 嵌入一张默认图）
        // 或者这里先放一个空 Vec，但 load_and_blur 需要处理空数据的情况
        let default_bytes =
            include_bytes!("/home/eternity/Music/专辑图片/无法解析:未知.jpeg").to_vec();
        let (_, blurred) = Self::load_and_blur(default_bytes.clone());

        Self {
            raw_bytes: default_bytes,
            blurred_handle: blurred,
        }
    }
}

impl BackGroundImage {
    /// 模糊处理：接收 Vec<u8> 并返回原始句柄和模糊句柄
    fn load_and_blur(bytes: Vec<u8>) -> (Handle, Handle) {
        // 使用 bytes 构建 iced 的原始句柄
        let raw_handle = Handle::from_bytes(bytes.clone());

        // 使用 Cursor 让 image 库从内存字节流中读取图片
        let blurred_handle = match ImageReader::new(Cursor::new(bytes)).with_guessed_format() {
            Ok(reader) => match reader.decode() {
                Ok(img) => {
                    let blurred_img = img.blur(15.0);
                    let width = blurred_img.width();
                    let height = blurred_img.height();
                    let rgba_bytes = blurred_img.to_rgba8().into_raw();

                    Handle::from_rgba(width, height, rgba_bytes)
                }
                Err(_) => raw_handle.clone(),
            },
            Err(_) => raw_handle.clone(),
        };

        (raw_handle, blurred_handle)
    }

    /// 更新
    pub fn update(&mut self, message: BackGroundImageMessage) {
        match message {
            BackGroundImageMessage::Set(bytes) => {
                self.raw_bytes = bytes;
                // 传入克隆的字节用于处理
                let (_, blurred) = Self::load_and_blur(self.raw_bytes.clone());
                self.blurred_handle = blurred;
            }
        }
    }

    /// 渲染
    pub fn view(&self) -> Element<'_, BackGroundImageMessage> {
        iced::widget::image(self.blurred_handle.clone())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .content_fit(iced::ContentFit::Cover)
            .into()
    }
}
