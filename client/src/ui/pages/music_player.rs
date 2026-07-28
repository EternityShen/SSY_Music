/// 播放器页面
pub struct MusicPlayer {
    music_slider: crate::ui::widgets::music_slider::MusicSlider,
    album_image: crate::ui::widgets::album_image::AlbumImage,
    music_lyric: crate::ui::widgets::music_lyric::MusicLyric,
    music_info: crate::ui::widgets::music_info::MusicInfo,
}

/// 消息
pub enum MusicPlayerMessage {
    MusicSlider(crate::ui::widgets::music_slider::MusicSliderMessage),
    AlbumImage(crate::ui::widgets::album_image::AlbumImageMessage),
    MusicLyric(crate::ui::widgets::music_lyric::MusicLyricMessage),
    MusicInfo(crate::ui::widgets::music_info::MusicInfoMessage),
}

impl MusicPlayer {
    /// 创建
    pub fn new() -> Self {
        let music_slider = crate::ui::widgets::music_slider::MusicSlider::default();
        let album_image = crate::ui::widgets::album_image::AlbumImage::default();
        let music_lyric = crate::ui::widgets::music_lyric::MusicLyric::default();
        let music_info = crate::ui::widgets::music_info::MusicInfo::default();

        Self {
            music_slider,
            album_image,
            music_lyric,
            music_info,
        }
    }

    /// 更新
    pub fn update(&mut self, message: MusicPlayerMessage) -> iced::Task<MusicPlayerMessage> {
        match message {
            MusicPlayerMessage::AlbumImage(a_m) => {
                self.album_image.update(a_m);
            }
            MusicPlayerMessage::MusicSlider(m_m) => {
                self.music_slider.update(m_m);
            }
            MusicPlayerMessage::MusicLyric(m_l) => {
                let task = self.music_lyric.update(m_l);
                return task.map(MusicPlayerMessage::MusicLyric);
            }
            MusicPlayerMessage::MusicInfo(m_i) => {
                self.music_info.update(m_i);
            }
        };

        iced::Task::none()
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, MusicPlayerMessage> {
        let album_image = self.album_image.view().map(MusicPlayerMessage::AlbumImage);
        let left = iced::widget::container(iced::widget::column![album_image])
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill);

        let music_info = self.music_info.view().map(MusicPlayerMessage::MusicInfo);
        let music_slider = self
            .music_slider
            .view()
            .map(MusicPlayerMessage::MusicSlider);
        let right = iced::widget::container(
            iced::widget::column![music_info, music_slider]
                .spacing(20)
                .align_x(iced::Alignment::Center),
        )
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill);

        let music_lyric = self.music_lyric.view().map(MusicPlayerMessage::MusicLyric);

        let top = iced::widget::row![
            iced::widget::space::horizontal(),
            iced::widget::container(music_lyric).width(600).height(70),
            iced::widget::space::horizontal()
        ]
        .height(90);

        let buttom = iced::widget::row![left, right]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .spacing(10);

        iced::widget::column![top, buttom].into()
    }
}

impl Default for MusicPlayer {
    /// 应付语法服务器
    fn default() -> Self {
        Self::new()
    }
}
