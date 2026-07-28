use std::time::Duration;

use futures_util::{FutureExt, SinkExt, StreamExt};

/// 页面枚举
pub enum Pages {
    Home,
    Player,
    MusicList,
    Help,
}

/// 应用窗口
pub struct ShenEternityMusicAppWindow {
    api: std::sync::Arc<crate::api::request::MusicClient>,
    play_index: u64,
    in_play: bool,
    music_list_num: u64,
    play_time: u64,
    page: Pages,
    background_image: super::widgets::background_image::BackGroundImage,
    music_list_page: super::pages::music_list::SongList,
    music_player_page: super::pages::music_player::MusicPlayer,
    player_sender: Option<iced::futures::channel::mpsc::Sender<super::event::PlayerHandleEvent>>,
}

/// 消息
pub enum ShenEternityMusicMessage {
    BackGroundImageMessage(super::widgets::background_image::BackGroundImageMessage),
    MusicListPageMessage(super::pages::music_list::SongListMessage),
    MusicPlayerPageMessage(super::pages::music_player::MusicPlayerMessage),
    PlayerEvent(super::event::PlayerEvent),
    SongBytes(Vec<u8>),
    SongData(Option<crate::api::data::Song>),
    ImageBytes(Option<Vec<u8>>),
    KeyPressed(iced::keyboard::Key),
}

/// 获取音乐数据
pub async fn get_song_bytes(
    api: std::sync::Arc<crate::api::request::MusicClient>,
    id: u64,
) -> Vec<u8> {
    let mut song_bytes = Vec::new();
    let stream = api.fetch_audio_stream(id).await.unwrap();

    tokio::pin!(stream);

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(byte) => {
                song_bytes.extend_from_slice(&byte);
            }
            Err(_e) => {}
        }
    }

    song_bytes
}

/// 获取专辑图片数据
pub async fn get_album_image_bytes(
    api: std::sync::Arc<crate::api::request::MusicClient>,
    id: u64,
) -> Option<Vec<u8>> {
    api.get_image(id).await
}

/// 获取歌曲数据
pub async fn get_song_data(
    api: std::sync::Arc<crate::api::request::MusicClient>,
    id: u64,
) -> Option<crate::api::data::Song> {
    api.fetch_song_data(id).await
}

impl ShenEternityMusicAppWindow {
    /// 创建
    pub fn new() -> Self {
        let mut logger = logger::Logger::new("./client.log");
        logger.clear();
        let logger_handle = std::sync::Arc::new(std::sync::Mutex::new(logger));
        let api_client = std::sync::Arc::new(crate::api::request::MusicClient::new(
            "127.0.0.1:3000",
            logger_handle.clone(),
        ));

        let background_image = super::widgets::background_image::BackGroundImage::default();
        let music_list_page = crate::ui::pages::music_list::SongList::new(api_client.clone());
        let music_player_page = crate::ui::pages::music_player::MusicPlayer::default();

        Self {
            api: api_client,
            play_index: 0,
            in_play: false,
            music_list_num: 0,
            play_time: 0,
            page: Pages::Player,
            background_image,
            music_list_page,
            music_player_page,
            player_sender: None,
        }
    }

    /// 更新
    pub fn update(
        &mut self,
        message: ShenEternityMusicMessage,
    ) -> iced::Task<ShenEternityMusicMessage> {
        match message {
            ShenEternityMusicMessage::BackGroundImageMessage(b_g_i) => {
                self.background_image.update(b_g_i);
            }
            ShenEternityMusicMessage::MusicListPageMessage(m_l) => {
                // 集想要触发的 Task
                let mut extra_tasks = Vec::new();

                if let super::pages::music_list::SongListMessage::ListItemOnPress(id) = m_l {
                    self.play_index = id;
                    self.in_play = true;
                    // 创建 Task
                    let get_song_task = iced::Task::perform(
                        get_song_bytes(self.api.clone(), id),
                        ShenEternityMusicMessage::SongBytes,
                    );

                    let image_task = iced::Task::perform(
                        get_album_image_bytes(self.api.clone(), id),
                        ShenEternityMusicMessage::ImageBytes,
                    );

                    let get_song_data = iced::Task::perform(
                        get_song_data(self.api.clone(), id),
                        ShenEternityMusicMessage::SongData,
                    );

                    let task = self
                        .music_player_page
                        .update(super::pages::music_player::MusicPlayerMessage::MusicLyric(
                            super::widgets::music_lyric::MusicLyricMessage::GetLyrics((
                                self.api.clone(),
                                id,
                            )),
                        ))
                        .map(ShenEternityMusicMessage::MusicPlayerPageMessage);

                    // 把 Task 存进数组里
                    extra_tasks.push(get_song_task);
                    extra_tasks.push(get_song_data);
                    extra_tasks.push(image_task);
                    extra_tasks.push(task);
                }

                if let super::pages::music_list::SongListMessage::SongsFetched(songs) = m_l.clone()
                {
                    self.music_list_num = songs.unwrap().len() as u64;
                }

                // 获取子页面原本要返回的 Task
                let page_task = self
                    .music_list_page
                    .update(m_l)
                    .map(ShenEternityMusicMessage::MusicListPageMessage);

                extra_tasks.push(page_task);

                // 把它们打成一个包 (Batch) 一起返回给 Iced 运行
                return iced::Task::batch(extra_tasks);
            }

            ShenEternityMusicMessage::MusicPlayerPageMessage(m_p) => match m_p {
                super::pages::music_player::MusicPlayerMessage::MusicSlider(
                    super::widgets::music_slider::MusicSliderMessage::Seek(value),
                ) => {
                    let _ = self
                        .player_sender
                        .clone()
                        .unwrap()
                        .try_send(crate::ui::event::PlayerHandleEvent::Seek(value as u64));
                }

                super::pages::music_player::MusicPlayerMessage::MusicLyric(m_l) => {
                    let task = self.music_player_page.update(
                        super::pages::music_player::MusicPlayerMessage::MusicLyric(m_l),
                    );

                    return task.map(ShenEternityMusicMessage::MusicPlayerPageMessage);
                }
                _ => {}
            },

            ShenEternityMusicMessage::PlayerEvent(event) => match event {
                super::event::PlayerEvent::Ready(sender) => {
                    self.player_sender = Some(sender);
                }
                super::event::PlayerEvent::MusicTime(time) => {
                    self.play_time = time;

                    let task_0 = self.music_player_page.update(
                        crate::ui::pages::music_player::MusicPlayerMessage::MusicSlider(
                            crate::ui::widgets::music_slider::MusicSliderMessage::Seek(time as f32),
                        ),
                    );

                    let task_1 = self.music_player_page.update(
                        super::pages::music_player::MusicPlayerMessage::MusicLyric(
                            super::widgets::music_lyric::MusicLyricMessage::Time(time as f32),
                        ),
                    );

                    let tasks = vec![
                        task_0.map(ShenEternityMusicMessage::MusicPlayerPageMessage),
                        task_1.map(ShenEternityMusicMessage::MusicPlayerPageMessage),
                    ];

                    return iced::Task::batch(tasks);
                }
                super::event::PlayerEvent::Next => {
                    let id = if self.play_index == self.music_list_num - 1 {
                        0
                    } else {
                        self.play_index + 1
                    };

                    self.play_index = id;

                    // 先收集想要触发的 Task
                    let mut extra_tasks = Vec::new();
                    // 创建 Task
                    let get_song_task = iced::Task::perform(
                        get_song_bytes(self.api.clone(), id),
                        ShenEternityMusicMessage::SongBytes,
                    );

                    let image_task = iced::Task::perform(
                        get_album_image_bytes(self.api.clone(), id),
                        ShenEternityMusicMessage::ImageBytes,
                    );

                    let get_song_data = iced::Task::perform(
                        get_song_data(self.api.clone(), id),
                        ShenEternityMusicMessage::SongData,
                    );

                    let task = self
                        .music_player_page
                        .update(super::pages::music_player::MusicPlayerMessage::MusicLyric(
                            super::widgets::music_lyric::MusicLyricMessage::GetLyrics((
                                self.api.clone(),
                                id,
                            )),
                        ))
                        .map(ShenEternityMusicMessage::MusicPlayerPageMessage);

                    // 把 Task 存进数组里
                    extra_tasks.push(get_song_task);
                    extra_tasks.push(get_song_data);
                    extra_tasks.push(image_task);
                    extra_tasks.push(task);

                    return iced::Task::batch(extra_tasks);
                }
            },

            ShenEternityMusicMessage::SongBytes(data) => {
                if let Some(mut sender) = self.player_sender.clone() {
                    let _ = sender.try_send(super::event::PlayerHandleEvent::PlayBytes(data));
                }
            }

            ShenEternityMusicMessage::SongData(data) => {
                if let Some(song) = data {
                    // 一堆music_player_page的update，和一堆task
                    let task_0 = self.music_player_page.update(
                        crate::ui::pages::music_player::MusicPlayerMessage::MusicSlider(
                            crate::ui::widgets::music_slider::MusicSliderMessage::SetAllValue(
                                song.duration as f32,
                            ),
                        ),
                    );

                    let task_1 = self.music_player_page.update(
                        crate::ui::pages::music_player::MusicPlayerMessage::MusicInfo(
                            crate::ui::widgets::music_info::MusicInfoMessage::Set((
                                song.title.clone(),
                                song.artist.clone(),
                            )),
                        ),
                    );

                    let task_2 = self.music_player_page.update(
                        crate::ui::pages::music_player::MusicPlayerMessage::AlbumImage(
                            crate::ui::widgets::album_image::AlbumImageMessage::SetTitle(
                                song.album.clone(),
                            ),
                        ),
                    );

                    // 打包
                    let tasks = vec![
                        task_0.map(ShenEternityMusicMessage::MusicPlayerPageMessage),
                        task_1.map(ShenEternityMusicMessage::MusicPlayerPageMessage),
                        task_2.map(ShenEternityMusicMessage::MusicPlayerPageMessage),
                    ];

                    // 返回
                    return iced::Task::batch(tasks);
                }
            }

            ShenEternityMusicMessage::ImageBytes(data) => {
                if let Some(image_date) = data {
                    self.background_image.update(
                        super::widgets::background_image::BackGroundImageMessage::Set(
                            image_date.clone(),
                        ),
                    );

                    let handle = iced::widget::image::Handle::from_bytes(image_date);

                    let task = self.music_player_page.update(
                        crate::ui::pages::music_player::MusicPlayerMessage::AlbumImage(
                            crate::ui::widgets::album_image::AlbumImageMessage::Set(handle),
                        ),
                    );

                    return task.map(ShenEternityMusicMessage::MusicPlayerPageMessage);
                }
            }

            ShenEternityMusicMessage::KeyPressed(key) => match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => match self.page {
                    Pages::Home => {
                        self.page = Pages::MusicList;
                    }
                    Pages::MusicList => {
                        self.page = Pages::Player;
                    }
                    Pages::Player => {
                        self.page = Pages::Help;
                    }
                    Pages::Help => {
                        self.page = Pages::Home;
                    }
                },

                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => {
                    let seek_value = self.play_time + 5;
                    let _ = self
                        .player_sender
                        .clone()
                        .unwrap()
                        .try_send(super::event::PlayerHandleEvent::Seek(seek_value));
                }

                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
                    let seek_value = self.play_time - 5;
                    let _ = self
                        .player_sender
                        .clone()
                        .unwrap()
                        .try_send(super::event::PlayerHandleEvent::Seek(seek_value));
                }

                iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                    if self.in_play {
                        self.in_play = false;
                        let _ = self
                            .player_sender
                            .clone()
                            .unwrap()
                            .try_send(super::event::PlayerHandleEvent::Pause);
                    } else {
                        self.in_play = true;
                        let _ = self
                            .player_sender
                            .clone()
                            .unwrap()
                            .try_send(super::event::PlayerHandleEvent::Play);
                    }
                }
                _ => {}
            },
        };
        iced::Task::none()
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, ShenEternityMusicMessage> {
        let background_image = self
            .background_image
            .view()
            .map(ShenEternityMusicMessage::BackGroundImageMessage);
        match self.page {
            Pages::Home => {
                let window = iced::widget::stack!(background_image);
                window.into()
            }

            Pages::Player => {
                let music_player_page = self
                    .music_player_page
                    .view()
                    .map(ShenEternityMusicMessage::MusicPlayerPageMessage);
                let window = iced::widget::stack!(background_image, music_player_page);
                window.into()
            }

            Pages::MusicList => {
                let music_list_page = self
                    .music_list_page
                    .view()
                    .map(ShenEternityMusicMessage::MusicListPageMessage);
                let window = iced::widget::stack!(background_image, music_list_page);
                window.into()
            }

            Pages::Help => {
                let window = iced::widget::stack!(background_image);
                window.into()
            }
        }
    }

    /// 订阅
    pub fn subscription(&self) -> iced::Subscription<ShenEternityMusicMessage> {
        iced::Subscription::batch(vec![
            iced::Subscription::run(player_work).map(ShenEternityMusicMessage::PlayerEvent),
            iced::event::listen_with(|event, _status, _window_id| match event {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
                    Some(ShenEternityMusicMessage::KeyPressed(key))
                }
                _ => None,
            }),
        ])
    }
}

/// 播放器的订阅构造器
fn player_work() -> impl iced::futures::Stream<Item = crate::ui::event::PlayerEvent> {
    iced::stream::channel(100, async |mut output| {
        let (sender, mut receiver) =
            iced::futures::channel::mpsc::channel::<crate::ui::event::PlayerHandleEvent>(100);

        let player = crate::player::handle::PlayerHandle::default();
        let mut in_play = false;

        let _ = output
            .send(crate::ui::event::PlayerEvent::Ready(sender))
            .await;

        let mut ticker = tokio::time::interval(Duration::from_millis(500));

        loop {
            iced::futures::select! {
                cmd = receiver.next().fuse() => {
                    if let Some(cmd) = cmd {
                        match cmd {
                            crate::ui::event::PlayerHandleEvent::Play => {
                                in_play = true;
                                player.play();
                            }
                            crate::ui::event::PlayerHandleEvent::Pause => {
                                in_play = false;
                                player.pause();
                            }
                            crate::ui::event::PlayerHandleEvent::PlayBytes(bytes) => {
                                in_play = true;
                                player.play_form_bytes(bytes);
                            }
                            crate::ui::event::PlayerHandleEvent::Seek(value) => {
                                player.seek(value);
                            }
                        }
                    }
                }

                _ = ticker.tick().fuse() => {
                    let time = player.get_pos_time();
                    let _ = output.send(crate::ui::event::PlayerEvent::MusicTime(time)).await;
                    if player.is_empty() & in_play{
                        let _ = output.send(crate::ui::event::PlayerEvent::Next).await;
                    }
                }
            }
        }
    })
}

impl Default for ShenEternityMusicAppWindow {
    /// 默认构造(纯应付语法服务器，不写会给警告)
    fn default() -> Self {
        Self::new()
    }
}
