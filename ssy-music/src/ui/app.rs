use futures_util::FutureExt;
use futures_util::SinkExt;
use futures_util::StreamExt;
use ringbuf::traits::Consumer;
use ringbuf::traits::Observer;
use ringbuf::traits::Split;
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{FrequencyLimit, samples_fft_to_spectrum};

use super::pages;
use super::player_manger;
use super::widgets;
use crate::api;
use crate::ui::event;

/// 播放模式
enum PlayMode {
    Net,
    Load,
}

/// 页面枚举
enum Page {
    Home,
    MusicList,
    Player,
    PlayList,
}

/// 整个ui的上帝
pub struct App {
    api: std::sync::Arc<api::request::MusicClient>,
    play_mode: PlayMode,
    background: widgets::background_image::BackGroundImage,
    music_list_page: pages::music_list::MusicListPage,
    play_list_page: pages::play_list::PlayListPage,
    home_page: pages::home::HomePage,
    page: Page,
    page_switch: widgets::page_switch::PageSwitch,
    player_page: pages::player::PlayerPage,
    player_manger: player_manger::PlayerManger,
}

/// 消息
pub enum AppMessage {
    BackGroundMessage(widgets::background_image::BackGroundImageMessage),
    PageSwitch(widgets::page_switch::PageSwitchMessage),
    MusicListMessage(pages::music_list::MusicListPageMessage),
    PlayerPageMessage(pages::player::PlayerPageMessage),
    HomePageMessage(pages::home::HomePageMessage),
    PlayListPageMessage(pages::play_list::PlayListPageMessage),
    KeyPressed(iced::keyboard::Key),
    Songs(Vec<(api::data::Song, Vec<u8>)>),
    Song((api::data::Song, Vec<u8>)),
    SongBytes(Vec<u8>),
    SongData(api::data::Song),
    ImageData(Option<Vec<u8>>),
    LyricData(Option<String>),
    AppEventMessage(event::app::AppEvent),
    Tick,
}

/// 获取音乐列表
async fn get_list(
    api: std::sync::Arc<api::request::MusicClient>,
    value: Option<String>,
) -> Vec<(api::data::Song, Vec<u8>)> {
    let songs = api.fetch_songs(value).await.unwrap_or_default();

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

async fn get_song_and_image(
    api: std::sync::Arc<api::request::MusicClient>,
    id: u64,
) -> (api::data::Song, Vec<u8>) {
    let song = get_song_data(api.clone(), id).await;
    let image = get_image_bytes(api.clone(), id).await.unwrap();

    (song, image)
}

impl App {
    /// 网络更新歌曲的Task,由于是纯逻辑,所以提出一个函数
    fn updata_song_net(
        &mut self,
        api: std::sync::Arc<api::request::MusicClient>,
        id: u64,
    ) -> iced::Task<AppMessage> {
        self.player_manger.list.push(id);

        self.player_manger.playing_idx = self.player_manger.get_index_form_id(id).unwrap();

        let get_song_bytes =
            iced::Task::perform(get_song_bytes(api.clone(), id), AppMessage::SongBytes);
        let get_image_bytes =
            iced::Task::perform(get_image_bytes(api.clone(), id), AppMessage::ImageData);
        let get_song_data =
            iced::Task::perform(get_song_data(api.clone(), id), AppMessage::SongData);
        let get_lyric_str =
            iced::Task::perform(get_lyric_data(api.clone(), id), AppMessage::LyricData);
        let get_song_and_image =
            iced::Task::perform(get_song_and_image(api.clone(), id), AppMessage::Song);
        iced::Task::batch(vec![
            get_song_bytes,
            get_image_bytes,
            get_song_data,
            get_lyric_str,
            get_song_and_image,
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
        self.player_page.set_album_title(song_data.album.clone());
        self.player_page.set_all_progress(song_data.duration as u64);
        let image_data = std::fs::read(song_data.image.clone()).unwrap_or_default();
        self.background
            .update(widgets::background_image::BackGroundImageMessage::Set(
                image_data.clone(),
            ));

        self.play_list_page
            .add_item((song_data.clone(), image_data));

        self.player_manger.list.push(id);

        let lyric_path = self
            .player_manger
            .load_data
            .lyrics_dir
            .join(format!("{}-{}.txt", song_data.title, song_data.artist));

        let lyric_str = std::fs::read_to_string(lyric_path).unwrap_or_default();

        self.player_manger.playing_idx = self.player_manger.get_index_form_id(id).unwrap();

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
        let page_switch = widgets::page_switch::PageSwitch::default();
        let home_page = pages::home::HomePage::default();
        let play_list_page = pages::play_list::PlayListPage::default();
        let player_manger =
            player_manger::PlayerManger::new(&config.load_db_path, &config.lyrics_path);

        Self {
            api: api_client,
            play_mode,
            background,
            music_list_page,
            home_page,
            play_list_page,
            page_switch,
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
            // 页面切换
            AppMessage::PageSwitch(message) => match message {
                widgets::page_switch::PageSwitchMessage::Left => match self.page {
                    Page::Home => {
                        self.page = Page::PlayList;
                    }
                    Page::MusicList => {
                        self.page = Page::Home;
                    }
                    Page::Player => {
                        self.page = Page::MusicList;
                    }
                    Page::PlayList => {
                        self.page = Page::Player;
                    }
                },
                widgets::page_switch::PageSwitchMessage::Right => match self.page {
                    Page::Home => {
                        self.page = Page::MusicList;
                    }
                    Page::MusicList => {
                        self.page = Page::Player;
                    }
                    Page::Player => {
                        self.page = Page::PlayList;
                    }
                    Page::PlayList => {
                        self.page = Page::Home;
                    }
                },
            },

            // -------------------------------------------------------------------------
            // 页面
            AppMessage::MusicListMessage(message) => {
                let (task, event) = self.music_list_page.update(message);
                let page_task = task.map(AppMessage::MusicListMessage);

                if let Some(event) = event {
                    match event {
                        pages::music_list::MusicListPageEvent::FetchSongs(value) => {
                            match self.play_mode {
                                PlayMode::Net => {
                                    let fetch_task = iced::Task::perform(
                                        get_list(self.api.clone(), value),
                                        AppMessage::Songs,
                                    );
                                    return iced::Task::batch(vec![page_task, fetch_task]);
                                }
                                PlayMode::Load => {
                                    self.player_manger.load_data.re_load();
                                    let songs_data =
                                        self.player_manger.load_data.search_songs(value);
                                    self.music_list_page.set_list_data(songs_data);
                                }
                            }
                        }
                        pages::music_list::MusicListPageEvent::SongSelected(id) => {
                            match self.play_mode {
                                PlayMode::Net => {
                                    let song_task = self.updata_song_net(self.api.clone(), id);
                                    return iced::Task::batch(vec![song_task, page_task]);
                                }
                                PlayMode::Load => {
                                    self.updata_song_load(id);
                                }
                            }
                        }
                        pages::music_list::MusicListPageEvent::PlayNext(id) => {
                            match self.play_mode {
                                PlayMode::Load => {
                                    let song =
                                        self.player_manger.load_data.get_song_data(id).unwrap();
                                    let image_path =
                                        self.player_manger.load_data.get_image_path(id).unwrap();

                                    let image_bytes = std::fs::read(image_path).unwrap();

                                    self.play_list_page.add_item((song, image_bytes));

                                    self.player_manger.list.push(id);
                                }
                                PlayMode::Net => {
                                    self.player_manger.list.push(id);

                                    return iced::Task::perform(
                                        get_song_and_image(self.api.clone(), id),
                                        AppMessage::Song,
                                    );
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

            AppMessage::PlayListPageMessage(message) => {
                let option = self.play_list_page.update(message);
                if let Some(event) = option {
                    match event {
                        pages::play_list::PlayListEvent::Play(id) => {
                            match self.play_mode {
                                PlayMode::Net => {
                                    return self.updata_song_net(self.api.clone(), id);
                                }
                                PlayMode::Load => {
                                    self.updata_song_load(id);
                                }
                            };
                        }
                        pages::play_list::PlayListEvent::Delete(id) => {
                            self.player_manger.remove_index_form_id(id);
                        }
                    }
                }
            }

            // -------------------------------------------------------------------------
            // 键盘
            AppMessage::KeyPressed(key) => match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => match self.page {
                    Page::Home => self.page = Page::MusicList,
                    Page::MusicList => self.page = Page::Player,
                    Page::Player => self.page = Page::PlayList,
                    Page::PlayList => self.page = Page::MusicList,
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

            AppMessage::Song(data) => {
                self.player_manger.list.push(data.0.id);
                self.play_list_page.add_item(data);
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
                event::app::AppEvent::FftReceiver(rx) => {
                    self.player_page.set_spectrum_rx(rx);
                }
                event::app::AppEvent::Next => {
                    let id = self.player_manger.next_id();

                    match self.play_mode {
                        PlayMode::Net => {
                            return self.updata_song_net(self.api.clone(), id);
                        }
                        PlayMode::Load => {
                            self.updata_song_load(id);
                        }
                    }
                }
            },

            // -------------------------------------------------------------------------
            // Tick 音频可视化的tick
            AppMessage::Tick => {
                let _ = self
                    .player_page
                    .update(pages::player::PlayerPageMessage::Tick);
            }
        };
        iced::Task::none()
    }

    /// 订阅
    pub fn subscription(&self) -> iced::Subscription<AppMessage> {
        iced::Subscription::batch(vec![
            iced::Subscription::run(player_work).map(AppMessage::AppEventMessage),
            iced::time::every(std::time::Duration::from_millis(10)).map(|_| AppMessage::Tick),
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
        let page_switch = self.page_switch.view().map(AppMessage::PageSwitch);
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
                iced::widget::stack!(background, middle_layer, home_page, page_switch).into()
            }
            Page::Player => {
                let player_page = self.player_page.view().map(AppMessage::PlayerPageMessage);
                iced::widget::stack!(background, middle_layer, player_page, page_switch).into()
            }
            Page::MusicList => {
                let music_list = self
                    .music_list_page
                    .view()
                    .map(AppMessage::MusicListMessage);
                iced::widget::stack!(background, middle_layer, music_list, page_switch).into()
            }
            Page::PlayList => {
                let play_list = self
                    .play_list_page
                    .view()
                    .map(AppMessage::PlayListPageMessage);
                iced::widget::stack!(background, middle_layer, play_list, page_switch).into()
            }
        }
    }
}

const FFT_SIZE: usize = 2048;

/// player订阅的构建器
fn player_work() -> impl iced::futures::Stream<Item = event::app::AppEvent> {
    iced::stream::channel(100, async |mut output| {
        let (sender, mut receiver) =
            iced::futures::channel::mpsc::channel::<event::player::PlayerEvent>(100);

        // 初始化缓冲区
        let ring_buffer = ringbuf::HeapRb::<f32>::new(FFT_SIZE * 8);
        let (producer, mut consumer) = ring_buffer.split();
        let producer_arc = std::sync::Arc::new(std::sync::Mutex::new(producer));

        let mut player_handle = crate::player::handle::PlayerHandle::new(producer_arc);

        let (fft_tx, fft_rx) = std::sync::mpsc::channel();

        // 傅里叶变换采集器
        std::thread::spawn(move || {
            let mut fft_buffer = vec![0.0f32; FFT_SIZE];
            let num_bands = 64;
            let sample_rate = 44100; // 默认采样率

            loop {
                if consumer.occupied_len() >= FFT_SIZE {
                    consumer.pop_slice(&mut fft_buffer);

                    let windowed = hann_window(&fft_buffer);

                    if let Ok(spectrum) = samples_fft_to_spectrum(
                        &windowed,
                        sample_rate,
                        FrequencyLimit::Range(20.0, 4000.0),
                        Some(&divide_by_N_sqrt),
                    ) {
                        let raw_data = spectrum.data();
                        if !raw_data.is_empty() {
                            let chunk_size = (raw_data.len() / num_bands).max(1);
                            let mut bands = Vec::with_capacity(num_bands);

                            for chunk in raw_data.chunks(chunk_size) {
                                let avg_amp: f32 = chunk.iter().map(|(_, a)| a.val()).sum::<f32>()
                                    / chunk.len() as f32;

                                let db = if avg_amp > 0.0 {
                                    20.0 * avg_amp.log10()
                                } else {
                                    -60.0
                                };
                                let normalized = ((db - (-60.0)) / 60.0).clamp(0.0, 1.0);
                                bands.push(normalized);
                            }

                            let _ = fft_tx.send(bands);
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        let _ = output.send(event::app::AppEvent::Ready(sender)).await;
        let _ = output.send(event::app::AppEvent::FftReceiver(fft_rx)).await;

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
