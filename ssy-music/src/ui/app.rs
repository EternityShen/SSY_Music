use futures_util::FutureExt;
use futures_util::SinkExt;
use futures_util::StreamExt;

use super::pages;
use super::widgets;
use crate::api;
use crate::ui::event;

/// 播放模式
enum PlayMode {
    Net,
    Load,
}

/// Player的管理者
struct PlayerManger {
    playersender: Option<iced::futures::channel::mpsc::Sender<super::event::player::PlayerEvent>>,
    load_data: api::load::LoadDate,
    playing_id: u64,
    volume: f32,
    list_num: u64,
    play_time: u64,
    is_play: bool,
}

impl PlayerManger {
    fn new(db_path: &str, lyrics_path: &str) -> Self {
        let load_data = api::load::LoadDate::load_data_from_toml(db_path, lyrics_path);
        Self {
            playersender: None,
            load_data,
            playing_id: 0,
            volume: 1.0,
            list_num: 0,
            play_time: 0,
            is_play: false,
        }
    }

    /// 设置 sender
    fn set_sender(
        &mut self,
        sender: iced::futures::channel::mpsc::Sender<event::player::PlayerEvent>,
    ) {
        self.playersender = Some(sender)
    }

    /// 播放字节数据
    fn play_bytes(&mut self, data: Vec<u8>) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::PlayBytes(data));
        self.is_play = true;
    }

    /// 播放指定路径的音频
    fn play_path(&mut self, path: String) {
        let _ = self
            .playersender
            .clone()
            .unwrap()
            .try_send(super::event::player::PlayerEvent::PlayPath(path));
    }

    /// 下一首的id
    fn next_id(&mut self) -> u64 {
        if self.playing_id >= self.list_num {
            self.playing_id = 0;
        } else {
            self.playing_id += 1;
        }
        self.playing_id
    }

    /// 播放
    fn play(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Play);
        self.is_play = true;
    }

    /// 暂停
    fn pause(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Pause);
        self.is_play = false;
    }

    /// 跳转
    fn seek(&self, time: u64) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Seek(time));
    }

    /// 快进5s
    fn seek_add_5(&self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Seek(self.play_time + 5));
    }

    /// 倒退5s
    fn seek_subtract_5(&self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Seek(
                self.play_time.saturating_sub(5),
            ));
    }

    /// 音量+5%
    fn volume_add_5(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::SetVolume(
                (self.volume + 0.05).clamp(0.0, 1.0),
            ));

        self.volume = (self.volume + 0.05).clamp(0.0, 1.0);
    }

    /// 音量-5%
    fn volume_subtract_5(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::SetVolume(
                (self.volume - 0.05).clamp(0.0, 1.0),
            ));

        self.volume = (self.volume - 0.05).clamp(0.0, 1.0);
    }
}

/// 页面枚举
enum Page {
    Home,
    MusicList,
    Player,
}

/// 整个ui的上帝
pub struct App {
    api: std::sync::Arc<api::request::MusicClient>,
    play_mode: PlayMode,
    background: widgets::background_image::BackGroundImage,
    music_list_page: pages::music_list::MusicListPage,
    home_page: pages::home::HomePage,
    page: Page,
    player_page: pages::player::PlayerPage,
    player_manger: PlayerManger,
}

/// 消息
pub enum AppMessage {
    BackGroundMessage(widgets::background_image::BackGroundImageMessage),
    MusicListMessage(pages::music_list::MusicListPageMessage),
    PlayerPageMessage(pages::player::PlayerPageMessage),
    HomePageMessage(pages::home::HomePageMessage),
    KeyPressed(iced::keyboard::Key),
    Songs(Vec<(api::data::Song, Vec<u8>)>),
    SongBytes(Vec<u8>),
    SongData(api::data::Song),
    ImageData(Option<Vec<u8>>),
    LyricData(Option<String>),
    AppEventMessage(event::app::AppEvent),
}

/// 获取音乐列表
async fn get_list(
    api: std::sync::Arc<api::request::MusicClient>,
) -> Vec<(api::data::Song, Vec<u8>)> {
    let songs = api.fetch_songs(None).await.unwrap_or_default();

    let futures = songs.into_iter().map(|song| {
        let api = api.clone();
        async move {
            let image_data = api.get_image(song.id).await.unwrap_or_default();
            (song, image_data)
        }
    });

    futures_util::future::join_all(futures).await
}

/// 获取音乐字节数据
async fn get_song_bytes(api: std::sync::Arc<crate::api::request::MusicClient>, id: u64) -> Vec<u8> {
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

/// 获取音乐数据
async fn get_song_data(api: std::sync::Arc<api::request::MusicClient>, id: u64) -> api::data::Song {
    let option = api.fetch_song_data(id).await;
    match option {
        Some(song) => song,
        None => api::data::Song {
            id,
            title: "".to_string(),
            artist: "".to_string(),
            album: "".to_string(),
            path: "".to_string(),
            image: "".to_string(),
            duration: 520.0,
        },
    }
}

/// 获取图片字节数据
async fn get_image_bytes(
    api: std::sync::Arc<crate::api::request::MusicClient>,
    id: u64,
) -> Option<Vec<u8>> {
    api.get_image(id).await
}

/// 获取歌词
async fn get_lyric_data(api: std::sync::Arc<api::request::MusicClient>, id: u64) -> Option<String> {
    api.fetch_lyrics(id).await
}

impl App {
    /// 网络更新歌曲的Task,由于是纯逻辑,所以提出一个函数
    fn updata_song_net(
        api: std::sync::Arc<api::request::MusicClient>,
        id: u64,
    ) -> iced::Task<AppMessage> {
        let get_song_bytes =
            iced::Task::perform(get_song_bytes(api.clone(), id), AppMessage::SongBytes);
        let get_image_bytes =
            iced::Task::perform(get_image_bytes(api.clone(), id), AppMessage::ImageData);
        let get_song_data =
            iced::Task::perform(get_song_data(api.clone(), id), AppMessage::SongData);
        let get_lyric_str =
            iced::Task::perform(get_lyric_data(api.clone(), id), AppMessage::LyricData);
        iced::Task::batch(vec![
            get_song_bytes,
            get_image_bytes,
            get_song_data,
            get_lyric_str,
        ])
    }

    /// 本地更新歌曲的逻辑
    fn updata_song_load(&mut self, id: u64) {
        let image_path = self.player_manger.load_data.get_image_path(id).unwrap();
        let song_data = self.player_manger.load_data.get_song_data(id).unwrap();

        self.player_manger.play_path(song_data.path.clone());

        self.player_page
            .set_info((song_data.title.clone(), song_data.artist.clone()));
        self.player_page.set_album_image_from_path(image_path);
        self.player_page.set_album_title(song_data.album);
        self.player_page.set_all_progress(song_data.duration as u64);
        let bg_image_data = std::fs::read(song_data.image).unwrap_or_default();
        self.background
            .update(widgets::background_image::BackGroundImageMessage::Set(
                bg_image_data,
            ));

        let lyric_path = self
            .player_manger
            .load_data
            .lyrics_dir
            .join(format!("{}-{}.txt", song_data.title, song_data.artist));

        let lyric_str = std::fs::read_to_string(lyric_path).unwrap_or_default();

        self.player_page.set_lyric_data(lyric_str);
    }

    /// 创建
    pub fn new() -> Self {
        let config = crate::config::Config::new();

        let play_mode = match config.play_mode.as_str() {
            "Load" => PlayMode::Load,
            "Net" => PlayMode::Net,
            _ => {
                eprintln!("配置中的模式错误:{}", config.play_mode);
                panic!();
            }
        };

        let mut logger = logger::Logger::new(&config.log_path);
        logger.clear();
        let logger_handle = std::sync::Arc::new(std::sync::Mutex::new(logger));
        let api_client = std::sync::Arc::new(api::request::MusicClient::new(
            &config.load_db_path,
            logger_handle.clone(),
        ));

        let background = widgets::background_image::BackGroundImage::default();
        let music_list_page = pages::music_list::MusicListPage::new();
        let player_page = pages::player::PlayerPage::default();
        let home_page = pages::home::HomePage::default();
        let player_manger = PlayerManger::new(&config.load_db_path, &config.lyrics_path);

        Self {
            api: api_client,
            play_mode,
            background,
            music_list_page,
            home_page,
            page: Page::Home,
            player_page,
            player_manger,
        }
    }

    // 更新
    pub fn update(&mut self, message: AppMessage) -> iced::Task<AppMessage> {
        match message {
            // -------------------------------------------------------------------------
            // 背景
            AppMessage::BackGroundMessage(message) => {
                self.background.update(message);
            }

            // -------------------------------------------------------------------------
            // 页面
            AppMessage::MusicListMessage(message) => {
                let (task, event) = self.music_list_page.update(message);
                let page_task = task.map(AppMessage::MusicListMessage);

                if let Some(event) = event {
                    match event {
                        pages::music_list::MusicListPageEvent::RefreshRequested => {
                            match self.play_mode {
                                PlayMode::Net => {
                                    let fetch_task = iced::Task::perform(
                                        get_list(self.api.clone()),
                                        AppMessage::Songs,
                                    );
                                    return iced::Task::batch(vec![page_task, fetch_task]);
                                }
                                PlayMode::Load => {
                                    self.player_manger.load_data.re_load();
                                    let songs_data =
                                        self.player_manger.load_data.get_all_song_data();
                                    let song_num = songs_data.len() - 1;
                                    self.music_list_page.set_list_data(songs_data);
                                    self.player_manger.list_num = song_num as u64;
                                }
                            }
                        }
                        pages::music_list::MusicListPageEvent::SongSelected(id) => {
                            match self.play_mode {
                                PlayMode::Net => {
                                    let song_task = App::updata_song_net(self.api.clone(), id);
                                    return iced::Task::batch(vec![song_task, page_task]);
                                }
                                PlayMode::Load => {
                                    self.updata_song_load(id);
                                }
                            }
                        }
                    }
                }
                return page_task;
            }

            AppMessage::PlayerPageMessage(message) => {
                let event = self.player_page.update(message);
                if let Some(event) = event {
                    match event {
                        pages::player::PlayerPageEvent::Seek(time) => {
                            self.player_manger.seek(time);
                        }
                    }
                }
            }

            AppMessage::HomePageMessage(message) => {
                self.home_page.updata(message);
            }

            // -------------------------------------------------------------------------
            // 键盘
            AppMessage::KeyPressed(key) => match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => match self.page {
                    Page::Home => self.page = Page::MusicList,
                    Page::MusicList => self.page = Page::Player,
                    Page::Player => self.page = Page::MusicList,
                },
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
                    self.player_manger.seek_subtract_5();
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => {
                    self.player_manger.seek_add_5();
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                    if self.player_manger.is_play {
                        self.player_manger.pause();
                    } else {
                        self.player_manger.play();
                    }
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                    self.player_manger.volume_add_5();
                    self.player_page.set_volume(self.player_manger.volume);
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    self.player_manger.volume_subtract_5();
                    self.player_page.set_volume(self.player_manger.volume);
                }
                _ => {}
            },

            // -------------------------------------------------------------------------
            // 网络事件
            AppMessage::Songs(songs) => {
                let (task, _event) =
                    self.music_list_page
                        .update(pages::music_list::MusicListPageMessage::Songs(
                            songs.clone(),
                        ));

                self.player_manger.list_num = (songs.len() - 1) as u64;

                return task.map(AppMessage::MusicListMessage);
            }

            AppMessage::SongBytes(data) => {
                self.player_manger.play_bytes(data);
            }

            AppMessage::SongData(data) => {
                self.player_page.set_album_title(data.album.clone());
                self.player_page.set_all_progress(data.duration as u64);
                self.player_page.set_info((data.title, data.artist));
            }

            AppMessage::ImageData(data) => {
                if let Some(image_data) = data {
                    self.background
                        .update(widgets::background_image::BackGroundImageMessage::Set(
                            image_data.clone(),
                        ));

                    self.player_page.set_album_image_from_data(image_data);
                }
            }

            AppMessage::LyricData(data) => {
                let lyric_str = data.unwrap_or_default();
                self.player_page.set_lyric_data(lyric_str);
            }

            // -------------------------------------------------------------------------
            // 订阅发送的事件
            AppMessage::AppEventMessage(event) => match event {
                event::app::AppEvent::Ready(sender) => {
                    self.player_manger.set_sender(sender);
                }
                event::app::AppEvent::PlayTime(time) => {
                    self.player_page.set_progress(time);
                    self.player_page.set_lyric_time(time);
                    self.player_manger.play_time = time;
                }
                event::app::AppEvent::Next => {
                    let id = self.player_manger.next_id();

                    match self.play_mode {
                        PlayMode::Net => {
                            return App::updata_song_net(self.api.clone(), id);
                        }
                        PlayMode::Load => {
                            self.updata_song_load(id);
                        }
                    }
                }
            },
        };
        iced::Task::none()
    }

    /// 订阅
    pub fn subscription(&self) -> iced::Subscription<AppMessage> {
        iced::Subscription::batch(vec![
            iced::Subscription::run(player_work).map(AppMessage::AppEventMessage),
            iced::event::listen_with(|event, _status, _window_id| match event {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
                    Some(AppMessage::KeyPressed(key))
                }
                _ => None,
            }),
        ])
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        let background = self.background.view().map(AppMessage::BackGroundMessage);
        let middle_layer = iced::widget::container(iced::widget::space::horizontal())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4).into()),
                ..Default::default()
            });

        match self.page {
            Page::Home => {
                let home_page = self.home_page.view().map(AppMessage::HomePageMessage);
                iced::widget::stack!(background, middle_layer, home_page).into()
            }
            Page::Player => {
                let player_page = self.player_page.view().map(AppMessage::PlayerPageMessage);
                iced::widget::stack!(background, middle_layer, player_page).into()
            }
            Page::MusicList => {
                let music_list = self
                    .music_list_page
                    .view()
                    .map(AppMessage::MusicListMessage);
                iced::widget::stack!(background, middle_layer, music_list).into()
            }
        }
    }
}

/// player订阅的构建器
fn player_work() -> impl iced::futures::Stream<Item = event::app::AppEvent> {
    iced::stream::channel(100, async |mut output| {
        let (sender, mut receiver) =
            iced::futures::channel::mpsc::channel::<event::player::PlayerEvent>(100);

        let player_handle = crate::player::handle::PlayerHandle::default();

        let _ = output.send(event::app::AppEvent::Ready(sender)).await;

        let mut is_playing = false;

        let mut ticker = tokio::time::interval(std::time::Duration::from_millis(500));

        loop {
            iced::futures::select! {
                cmd = receiver.next().fuse() => {
                    if let Some(cmd) = cmd {
                        match cmd {
                            event::player::PlayerEvent::Play => {
                                is_playing = true;
                                player_handle.play();
                            }
                            event::player::PlayerEvent::Pause => {
                                is_playing = false;
                                player_handle.pause();
                            }
                            event::player::PlayerEvent::PlayBytes(data) => {
                                is_playing = true;
                                player_handle.play_form_bytes(data);
                            }
                            event::player::PlayerEvent::Seek(time) => {
                                player_handle.seek(time);
                            }
                            event::player::PlayerEvent::PlayPath(path) => {
                                is_playing = true;
                                player_handle.play_path(path);
                            }
                            event::player::PlayerEvent::SetVolume(value) => {
                                player_handle.set_volume(value);
                            }
                        }
                    }
                }

                _ = ticker.tick().fuse() => {
                    if is_playing {
                        let time = player_handle.get_pos_time();
                        let _ = output.send(event::app::AppEvent::PlayTime(time)).await;

                        if player_handle.is_empty() {
                            let _ = output.send(event::app::AppEvent::Next).await;
                        }
                    }
                }
            }
        }
    })
}

impl Default for App {
    /// 应付
    fn default() -> Self {
        Self::new()
    }
}
