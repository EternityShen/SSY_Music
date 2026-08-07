use std::path::{Path, PathBuf};

use iced::widget::button;
use serde::{Deserialize, Serialize};

use crate::ui::widgets;

/// 播放列表页面
pub struct PlayListPage {
    song_items: Vec<widgets::play_list_song_item::PlayListSongItem>,
    list_item: Vec<widgets::play_list_item::PlayListItem>,
    ids: Vec<u64>,
    listpath: PathBuf,
    songlist_o_c: bool,
    text_input: String,
}

/// 播放列表事件
pub enum PlayListEvent {
    Play(u64),
    Delete(u64),
    LoadList(Vec<u64>),
}

/// 消息
#[derive(Debug, Clone)]
pub enum PlayListPageMessage {
    SongOnPress(widgets::play_list_song_item::PlayListSongItemMessage),
    ListOnPress(widgets::play_list_item::PlayListItemMessage),
    TextInput(String),
    OpenCloseSidebar,
    SaveList,
}

#[derive(Serialize, Deserialize)]
struct Data {
    ids: Vec<u64>,
}

impl PlayListPage {
    /// 创建
    pub fn new() -> Self {
        let config_dir = crate::utils::get_user_config_dir_path();

        let mut list_item = Vec::new();
        let result = std::fs::read_dir(config_dir.join("list/"));
        match result {
            Ok(dir) => {
                for d in dir {
                    match d {
                        Ok(d_e) => {
                            if d_e.path().is_file() {
                                let len = d_e.file_name().to_string_lossy().len();
                                let item = widgets::play_list_item::PlayListItem::new(
                                    d_e.file_name().to_string_lossy()[..(len - 5)].to_string(),
                                    d_e.path().to_string_lossy().to_string(),
                                );
                                list_item.push(item);
                            }
                        }
                        Err(_e) => {}
                    }
                }
            }
            Err(_e) => {}
        }

        Self {
            song_items: Vec::new(),
            list_item,
            ids: Vec::new(),
            listpath: config_dir.join("list/"),
            songlist_o_c: false,
            text_input: String::new(),
        }
    }

    /// 更新
    pub fn update(&mut self, message: PlayListPageMessage) -> Option<PlayListEvent> {
        match message {
            PlayListPageMessage::SongOnPress(message) => match message {
                widgets::play_list_song_item::PlayListSongItemMessage::Delete(id) => {
                    if let Some(pos) = self.song_items.iter().position(|item| item.id == id) {
                        self.song_items.remove(pos);
                    }
                    Some(PlayListEvent::Delete(id))
                }
                widgets::play_list_song_item::PlayListSongItemMessage::OnPress(id) => {
                    Some(PlayListEvent::Play(id))
                }
            },
            PlayListPageMessage::SaveList => {
                let path = self.listpath.join(format!("{}.toml", self.text_input));
                self.save(path);

                let mut list_item = Vec::new();
                let result = std::fs::read_dir(self.listpath.clone());
                match result {
                    Ok(dir) => {
                        for d in dir {
                            match d {
                                Ok(d_e) => {
                                    if d_e.path().is_file() {
                                        let len = d_e.file_name().to_string_lossy().len();
                                        let item = widgets::play_list_item::PlayListItem::new(
                                            d_e.file_name().to_string_lossy()[..(len - 5)]
                                                .to_string(),
                                            d_e.path().to_string_lossy().to_string(),
                                        );
                                        list_item.push(item);
                                    }
                                }
                                Err(_e) => {}
                            }
                        }
                    }
                    Err(_e) => {}
                }

                self.list_item = list_item;

                None
            }
            PlayListPageMessage::ListOnPress(message) => match message {
                widgets::play_list_item::PlayListItemMessage::OnPress(path) => {
                    let path = self.listpath.join(path);
                    let content = std::fs::read_to_string(path).unwrap();
                    let data: Data = toml::from_str(&content).unwrap();

                    self.song_items.clear();
                    self.ids.clear();

                    self.songlist_o_c = false;

                    Some(PlayListEvent::LoadList(data.ids))
                }
            },
            PlayListPageMessage::OpenCloseSidebar => {
                self.songlist_o_c = !self.songlist_o_c;
                None
            }
            PlayListPageMessage::TextInput(text) => {
                self.text_input = text;
                None
            }
        }
    }

    /// 加项
    pub fn add_item(&mut self, data: (crate::api::data::Song, Vec<u8>)) {
        for id in &self.ids {
            if id == &data.0.id {
                return;
            }
        }
        self.ids.push(data.0.id);
        let item = widgets::play_list_song_item::PlayListSongItem::new(data.0, data.1);
        self.song_items.push(item);
    }

    /// 保存歌单
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let data = Data {
            ids: self.ids.clone(),
        };
        let toml = toml::to_string_pretty(&data).unwrap();
        std::fs::write(path, toml).expect("无法保存歌单");
    }

    /// 读取歌单
    pub fn load<P: AsRef<Path>>(&mut self, path: P) {
        let content = std::fs::read_to_string(path).unwrap();
        let data: Data = toml::from_str(&content).expect("无法对歌单进行转换");
        self.ids = data.ids;
        self.song_items.clear();
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, PlayListPageMessage> {
        let play_list_content =
            self.song_items
                .iter()
                .fold(iced::widget::column![].spacing(6), |col, item| {
                    let item_element = item.view().map(PlayListPageMessage::SongOnPress);
                    col.push(item_element)
                });

        let scrollable_play_list = iced::widget::scrollable(play_list_content)
            .style(|theme, status| iced::widget::scrollable::Style {
                container: iced::widget::container::Style {
                    background: Some(iced::Color::TRANSPARENT.into()),
                    ..Default::default()
                },
                ..iced::widget::scrollable::default(theme, status)
            })
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let songlist_list_content =
            self.list_item
                .iter()
                .fold(iced::widget::column![].spacing(10), |col, item| {
                    let item_element = item.view().map(PlayListPageMessage::ListOnPress);
                    col.push(item_element)
                });

        let scrollable_songlist_list = iced::widget::scrollable(songlist_list_content)
            .style(|theme, status| iced::widget::scrollable::Style {
                container: iced::widget::container::Style {
                    background: Some(iced::Color::TRANSPARENT.into()),
                    ..Default::default()
                },
                ..iced::widget::scrollable::default(theme, status)
            })
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let songlist_list = iced::widget::stack!(
            iced::widget::container(iced::widget::space::horizontal())
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .style(|_theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.6).into()),
                    ..Default::default()
                }),
            scrollable_songlist_list
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill);

        let save = button(iced::widget::text("save").size(20))
            .on_press(PlayListPageMessage::SaveList)
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
            });

        let open_close = button(iced::widget::text("O").size(20))
            .on_press(PlayListPageMessage::OpenCloseSidebar)
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
            });

        let text_input = iced::widget::container(
            iced::widget::text_input("输入歌单名", &self.text_input)
                .on_input(PlayListPageMessage::TextInput)
                .size(20)
                .style(|_theme, status| match status {
                    iced::widget::text_input::Status::Active => iced::widget::text_input::Style {
                        background: iced::Background::Color(iced::Color::from_rgba(
                            0.1, 0.1, 0.1, 0.0,
                        )),
                        border: iced::Border {
                            width: 1.0,
                            radius: 10.0.into(),
                            color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                        },
                        icon: iced::Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                        placeholder: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.3),
                        value: iced::Color::from_rgba(1.0, 1.0, 1.0, 1.0),
                        selection: iced::Color::from_rgba(0.1, 0.1, 0.1, 0.7),
                    },
                    iced::widget::text_input::Status::Hovered => iced::widget::text_input::Style {
                        background: iced::Background::Color(iced::Color::from_rgba(
                            0.3, 0.3, 0.3, 0.6,
                        )),
                        border: iced::Border {
                            width: 1.0,
                            radius: 10.0.into(),
                            color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                        },
                        icon: iced::Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                        placeholder: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.3),
                        value: iced::Color::from_rgba(1.0, 1.0, 1.0, 1.0),
                        selection: iced::Color::from_rgba(0.1, 0.1, 0.1, 0.7),
                    },
                    _ => iced::widget::text_input::Style {
                        background: iced::Background::Color(iced::Color::from_rgba(
                            0.1, 0.1, 0.1, 0.0,
                        )),
                        border: iced::Border {
                            width: 1.0,
                            radius: 10.0.into(),
                            color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                        },
                        icon: iced::Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                        placeholder: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.3),
                        value: iced::Color::from_rgba(1.0, 1.0, 1.0, 1.0),
                        selection: iced::Color::from_rgba(0.1, 0.1, 0.1, 0.7),
                    },
                }),
        )
        .width(100);

        let button_bar = iced::widget::column![
            iced::widget::space::vertical(),
            iced::widget::row![
                iced::widget::space::horizontal(),
                save,
                open_close,
                text_input
            ]
        ];

        if self.songlist_o_c {
            let middle_layer = button(iced::widget::space::horizontal())
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .on_press(PlayListPageMessage::OpenCloseSidebar)
                .style(|_theme, status| match status {
                    button::Status::Active => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.6, 0.6, 0.6, 0.2,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                    button::Status::Hovered => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.6, 0.6, 0.6, 0.2,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.6, 0.6, 0.6, 0.2,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                    button::Status::Disabled => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.6, 0.6, 0.6, 0.2,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                });

            let sidebar = iced::widget::row![iced::widget::space::horizontal(), songlist_list];

            iced::widget::stack!(scrollable_play_list, middle_layer, sidebar, button_bar).into()
        } else {
            iced::widget::stack!(scrollable_play_list, button_bar).into()
        }
    }
}

impl Default for PlayListPage {
    /// 应付
    fn default() -> Self {
        Self::new()
    }
}
