use iced::widget::button;

use crate::api;
use crate::ui::widgets;
use crate::ui::widgets::music_list_song_item::MusicListSongItemMessage;

/// 音乐列表页面
pub struct MusicListPage {
    items: Vec<widgets::music_list_song_item::MusicListSongItem>,
    input_text: String,
}

///  消息
#[derive(Clone)]
pub enum MusicListPageMessage {
    OnPress(MusicListSongItemMessage),
    FetchSongs(Option<String>),
    Songs(Vec<(api::data::Song, Vec<u8>)>),
    InputText(String),
}

/// 事件
pub enum MusicListPageEvent {
    SongSelected(u64),
    PlayNext(u64),
    FetchSongs(Option<String>),
}

impl MusicListPage {
    /// 创建
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            input_text: String::new(),
        }
    }

    /// 更新
    ///     -> (iced::Task<MusicListPageMessage>, Option<MusicListPageEvent>)
    pub fn update(
        &mut self,
        message: MusicListPageMessage,
    ) -> (iced::Task<MusicListPageMessage>, Option<MusicListPageEvent>) {
        match message {
            MusicListPageMessage::OnPress(message) => match message {
                MusicListSongItemMessage::OnPress(id) => (
                    iced::Task::none(),
                    Some(MusicListPageEvent::SongSelected(id)),
                ),
                MusicListSongItemMessage::PlayNext(id) => {
                    (iced::Task::none(), Some(MusicListPageEvent::PlayNext(id)))
                }
            },
            MusicListPageMessage::FetchSongs(value) => (
                iced::Task::none(),
                Some(MusicListPageEvent::FetchSongs(value)),
            ),
            MusicListPageMessage::Songs(data) => {
                self.items = data
                    .iter()
                    .map(|(song, image)| {
                        widgets::music_list_song_item::MusicListSongItem::new(
                            song.clone(),
                            image.clone(),
                        )
                    })
                    .collect();
                (iced::Task::none(), None)
            }
            MusicListPageMessage::InputText(text) => {
                self.input_text = text.clone();
                (
                    iced::Task::none(),
                    Some(MusicListPageEvent::FetchSongs(Some(text))),
                )
            }
        }
    }

    pub fn set_list_data(&mut self, data: Vec<api::data::Song>) {
        let mut item_data = Vec::new();

        for song in data {
            let image_bytes = std::fs::read(&song.image).unwrap_or_default();
            item_data.push((song.clone(), image_bytes));
        }

        let _ = self.update(MusicListPageMessage::Songs(item_data));
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, MusicListPageMessage> {
        let list_content =
            self.items
                .iter()
                .fold(iced::widget::column![].spacing(6), |col, item| {
                    let item_element = item.view().map(MusicListPageMessage::OnPress);
                    col.push(item_element)
                });

        let scrollable_list = iced::widget::scrollable(list_content)
            .style(|theme, status| iced::widget::scrollable::Style {
                container: iced::widget::container::Style {
                    background: Some(iced::Color::TRANSPARENT.into()),
                    ..Default::default()
                },
                ..iced::widget::scrollable::default(theme, status)
            })
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let bush = iced::widget::container(
            button(iced::widget::text("刷新").size(20))
                .on_press(MusicListPageMessage::FetchSongs(Some(
                    self.input_text.clone(),
                )))
                .style(|_theme, status| match status {
                    button::Status::Active => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.2, 0.2, 0.2, 0.2,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border {
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                    button::Status::Hovered => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.3, 0.3, 0.3, 0.3,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border {
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.5, 0.5, 0.5, 0.5,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border {
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                    button::Status::Disabled => button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.6, 0.6, 0.6, 0.6,
                        ))),
                        text_color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                        border: iced::Border {
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        shadow: iced::Shadow::default(),
                        snap: true,
                    },
                }),
        )
        .center_y(iced::Length::Fill);

        let text_input = iced::widget::container(
            iced::widget::text_input("歌名/歌手", &self.input_text)
                .on_input(MusicListPageMessage::InputText)
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
        .center_y(iced::Length::Fill);

        let top_bar = iced::widget::column![iced::widget::row![
            iced::widget::space::horizontal(),
            text_input,
            bush,
            iced::widget::space::horizontal()
        ]]
        .height(35);

        iced::widget::column![top_bar, scrollable_list].into()
    }
}

impl Default for MusicListPage {
    /// 应付
    fn default() -> Self {
        Self::new()
    }
}
