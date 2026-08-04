use iced::widget::button;

use crate::api;
use crate::ui::widgets;
use crate::ui::widgets::list_item::ListItemMessage;

/// 音乐列表页面
pub struct MusicListPage {
    items: Vec<widgets::list_item::ListItem>,
}

///  消息
#[derive(Clone)]
pub enum MusicListPageMessage {
    OnPress(ListItemMessage),
    FetchSongs,
    Songs(Vec<(api::data::Song, Vec<u8>)>),
}

/// 事件
pub enum MusicListPageEvent {
    SongSelected(u64),
    RefreshRequested,
}

impl MusicListPage {
    /// 创建
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 更新
    ///     -> (iced::Task<MusicListPageMessage>, Option<MusicListPageEvent>)
    pub fn update(
        &mut self,
        message: MusicListPageMessage,
    ) -> (iced::Task<MusicListPageMessage>, Option<MusicListPageEvent>) {
        match message {
            MusicListPageMessage::OnPress(message) => match message {
                ListItemMessage::OnPress(id) => (
                    iced::Task::none(),
                    Some(MusicListPageEvent::SongSelected(id)),
                ),
            },
            MusicListPageMessage::FetchSongs => (
                iced::Task::none(),
                Some(MusicListPageEvent::RefreshRequested),
            ),
            MusicListPageMessage::Songs(data) => {
                self.items = data
                    .iter()
                    .map(|(song, image)| {
                        widgets::list_item::ListItem::new(song.clone(), image.clone())
                    })
                    .collect();
                (iced::Task::none(), None)
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
                    let item_element = item.view().map(|message| match message {
                        ListItemMessage::OnPress(_id) => MusicListPageMessage::OnPress(message),
                    });
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

        let bush = button(iced::widget::text("刷新").size(30))
            .on_press(MusicListPageMessage::FetchSongs)
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
            });

        let bush_button = iced::widget::column![
            iced::widget::space::vertical(),
            iced::widget::row![
                iced::widget::space::horizontal(),
                iced::widget::container(bush)
            ]
        ];

        iced::widget::stack!(scrollable_list, bush_button).into()
    }
}

impl Default for MusicListPage {
    /// 应付
    fn default() -> Self {
        Self::new()
    }
}
