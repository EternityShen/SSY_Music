use crate::ui::widgets;

/// 音乐播放器页面
pub struct PlayerPage {
    album: widgets::album::Album,
    progress_bar: widgets::progress_bar::ProgressBar,
    info: widgets::info::Info,
}

/// 消息
pub enum PlayerPageMessage {
    AlbumMessage(widgets::album::AlbumMessage),
    ProgressBarMessage(widgets::progress_bar::ProgressBarMessage),
    InfoMessage(widgets::info::InfoMessage),
}

/// 事件
pub enum PlayerPageEvent {
    Seek(u64),
}

impl PlayerPage {
    /// 创建
    pub fn new() -> Self {
        let album = widgets::album::Album::default();
        let progress_bar = widgets::progress_bar::ProgressBar::default();
        let info = widgets::info::Info::default();
        Self {
            album,
            progress_bar,
            info,
        }
    }

    /// 更新
    ///     -> Option<PlayerPageEvent>
    pub fn update(&mut self, message: PlayerPageMessage) -> Option<PlayerPageEvent> {
        match message {
            PlayerPageMessage::AlbumMessage(message) => {
                self.album.update(message);
                None
            }
            PlayerPageMessage::ProgressBarMessage(message) => match message {
                widgets::progress_bar::ProgressBarMessage::Seek(time) => {
                    self.progress_bar.update(message);
                    Some(PlayerPageEvent::Seek(time as u64))
                }
                widgets::progress_bar::ProgressBarMessage::SetAllValue(_) => {
                    self.progress_bar.update(message);
                    None
                }
            },
            PlayerPageMessage::InfoMessage(message) => {
                self.info.update(message);
                None
            }
        }
    }

    // 一堆set,不多注释,一看就知道干嘛的
    pub fn set_progress(&mut self, time: u64) {
        self.progress_bar
            .update(widgets::progress_bar::ProgressBarMessage::Seek(time as f32));
    }

    pub fn set_album_image(&mut self, data: Vec<u8>) {
        let image_handle = iced::widget::image::Handle::from_bytes(data);
        self.album
            .update(widgets::album::AlbumMessage::Set(image_handle));
    }

    pub fn set_album_title(&mut self, title: String) {
        self.album
            .update(widgets::album::AlbumMessage::SetTitle(title));
    }

    pub fn set_all_progress(&mut self, time: u64) {
        self.progress_bar
            .update(widgets::progress_bar::ProgressBarMessage::SetAllValue(
                time as f32,
            ));
    }

    pub fn set_info(&mut self, data: (String, String)) {
        self.info.update(widgets::info::InfoMessage::Set(data));
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, PlayerPageMessage> {
        let album = self.album.view().map(PlayerPageMessage::AlbumMessage);
        let progress_bar = self
            .progress_bar
            .view()
            .map(PlayerPageMessage::ProgressBarMessage);
        let info = self.info.view().map(PlayerPageMessage::InfoMessage);

        let left =
            iced::widget::container(iced::widget::column![album].align_x(iced::Alignment::Center))
                .center_x(iced::Length::Fill)
                .center_y(iced::Length::Fill);

        let right = iced::widget::container(
            iced::widget::column![info, progress_bar]
                .spacing(15)
                .align_x(iced::Alignment::Center),
        )
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill);

        iced::widget::row![left, right]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

impl Default for PlayerPage {
    /// 应付
    fn default() -> Self {
        Self::new()
    }
}
