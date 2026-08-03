use crate::ui::widgets;

/// 音乐播放器页面
pub struct PlayerPage {
    album: widgets::album::Album,
    progress_bar: widgets::progress_bar::ProgressBar,
    info: widgets::info::Info,
    lyric: widgets::lyric::Lyric,

    spectrum_rx: Option<std::sync::mpsc::Receiver<Vec<f32>>>,
    spectrum_data: Vec<f32>,
    phase: f32,
}

/// 消息
pub enum PlayerPageMessage {
    AlbumMessage(widgets::album::AlbumMessage),
    ProgressBarMessage(widgets::progress_bar::ProgressBarMessage),
    InfoMessage(widgets::info::InfoMessage),
    LyricMessage(widgets::lyric::LyricMessage),

    Tick,
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
        let lyric = widgets::lyric::Lyric::default();

        Self {
            album,
            progress_bar,
            info,
            lyric,
            spectrum_rx: None,
            spectrum_data: vec![0.0; 64], // 默认 64 频段
            phase: 0.0,
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
                widgets::progress_bar::ProgressBarMessage::Seek(_time) => {
                    self.progress_bar.update(message);
                    None
                }
                widgets::progress_bar::ProgressBarMessage::OnSeek(_time) => {
                    self.progress_bar.update(message);
                    None
                }
                widgets::progress_bar::ProgressBarMessage::SetAllValue(_time) => {
                    self.progress_bar.update(message);
                    None
                }
                widgets::progress_bar::ProgressBarMessage::SetVolume(_value) => {
                    self.progress_bar.update(message);
                    None
                }
                widgets::progress_bar::ProgressBarMessage::OnRelease => {
                    let value = self.progress_bar.update(message);
                    if let Some(value) = value {
                        return Some(PlayerPageEvent::Seek(value as u64));
                    }
                    None
                }
            },
            PlayerPageMessage::InfoMessage(message) => {
                self.info.update(message);
                None
            }
            PlayerPageMessage::LyricMessage(message) => {
                self.lyric.update(message);
                None
            }
            PlayerPageMessage::Tick => {
                self.phase += 0.05;

                if let Some(rx) = &self.spectrum_rx {
                    while let Ok(new_data) = rx.try_recv() {
                        self.spectrum_data = new_data;
                    }
                }
                None
            }
        }
    }

    // 一堆set,不多注释,一看就知道干嘛的
    pub fn set_progress(&mut self, time: u64) {
        self.progress_bar
            .update(widgets::progress_bar::ProgressBarMessage::Seek(time as f32));
    }

    pub fn set_album_image_from_data(&mut self, data: Vec<u8>) {
        let image_handle = iced::widget::image::Handle::from_bytes(data);
        self.album
            .update(widgets::album::AlbumMessage::Set(image_handle));
    }

    pub fn set_album_image_from_path(&mut self, path: String) {
        let image_handle = iced::widget::image::Handle::from_path(path);
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

    pub fn set_volume(&mut self, value: f32) {
        self.progress_bar
            .update(widgets::progress_bar::ProgressBarMessage::SetVolume(value));
    }

    pub fn set_lyric_data(&mut self, data: String) {
        self.lyric
            .update(widgets::lyric::LyricMessage::SetLyrics(data));
    }

    pub fn set_lyric_time(&mut self, time: u64) {
        self.lyric
            .update(widgets::lyric::LyricMessage::SetTime(time));
    }

    pub fn set_spectrum_rx(&mut self, rx: std::sync::mpsc::Receiver<Vec<f32>>) {
        self.spectrum_rx = Some(rx);
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, PlayerPageMessage> {
        let album = self.album.view().map(PlayerPageMessage::AlbumMessage);
        let progress_bar = self
            .progress_bar
            .view()
            .map(PlayerPageMessage::ProgressBarMessage);
        let info = self.info.view().map(PlayerPageMessage::InfoMessage);
        let lyric = self.lyric.view().map(PlayerPageMessage::LyricMessage);

        let sine_wave_canvas = iced::widget::canvas(widgets::sine_wave_v::SineWave::new(
            &self.spectrum_data,
            self.phase,
        ))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill);

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

        let back = iced::widget::row![left, right]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let top = iced::widget::column![
            iced::widget::space::vertical(),
            lyric,
            iced::widget::space::Space::default().height(20),
        ]
        .width(iced::Length::Fill);

        iced::widget::stack!(sine_wave_canvas, back, top)
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
