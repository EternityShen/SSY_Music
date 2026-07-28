/// 音乐列表
pub struct SongList {
    api_client: std::sync::Arc<crate::api::request::MusicClient>,
    items: Vec<crate::ui::widgets::music_list_item::MusicListItem>,
}

/// 消息
#[derive(Debug, Clone)]
pub enum SongListMessage {
    ListItemOnPress(u64),
    SongsFetched(Option<Vec<(crate::api::data::Song, Vec<u8>)>>),
    FetchSongs,
}

impl SongList {
    /// 创建
    pub fn new(api_client: std::sync::Arc<crate::api::request::MusicClient>) -> Self {
        Self {
            api_client: api_client.clone(),
            items: Vec::new(),
        }
    }

    /// 获取所有歌曲数据的异步函数
    ///     Option<Vec<(crate::api::data::Song, Vec<u8>)>>
    async fn fetch_songs_with_covers(
        client: std::sync::Arc<crate::api::request::MusicClient>,
        keyword: Option<String>,
    ) -> Option<Vec<(crate::api::data::Song, Vec<u8>)>> {
        let songs = client.fetch_songs(keyword).await?;

        // 预先分配内存
        let mut result = Vec::with_capacity(songs.len());

        for song in songs {
            let image_data = client.get_image(song.id).await.unwrap_or_default();
            result.push((song, image_data));
        }

        Some(result)
    }

    /// 更新
    ///     -> iced::Task<SongListMessage>
    pub fn update(&mut self, message: SongListMessage) -> iced::Task<SongListMessage> {
        match message {
            SongListMessage::ListItemOnPress(_id) => {}
            SongListMessage::FetchSongs => {
                let client = self.api_client.clone();
                // Task的逻辑: 传入一个 future(async函数调用) 和 一个 Message(future的返回值类型)
                // Task会把 future 返回的东西扔进 Message() 里面, 然后再发给 update
                // 个人理解并记录(太他妈复杂了)
                return iced::Task::perform(
                    Self::fetch_songs_with_covers(client, None),
                    SongListMessage::SongsFetched,
                );
            }
            SongListMessage::SongsFetched(song_data) => {
                self.items = song_data
                    .unwrap()
                    .into_iter()
                    .map(|(song, image)| {
                        let image_handle = iced::widget::image::Handle::from_bytes(image);
                        crate::ui::widgets::music_list_item::MusicListItem::new(song, image_handle)
                    })
                    .collect();
            }
        };
        iced::Task::none()
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, SongListMessage> {
        let list_content =
            self.items
                .iter()
                .fold(iced::widget::column![].spacing(6), |col, item| {
                    let item_element = item.view().map(|message| match message {
                        crate::ui::widgets::music_list_item::MusicListItemMessage::OnPress(id) => {
                            SongListMessage::ListItemOnPress(id)
                        }
                    });
                    col.push(item_element)
                });

        let scrollable_list = iced::widget::scrollable(list_content).style(|theme, status| {
            iced::widget::scrollable::Style {
                container: iced::widget::container::Style {
                    background: Some(iced::Color::TRANSPARENT.into()),
                    ..Default::default()
                },
                ..iced::widget::scrollable::default(theme, status)
            }
        });
        let button = iced::widget::button(iced::widget::text("刷新").size(10))
            .on_press(SongListMessage::FetchSongs);
        iced::widget::column![scrollable_list, button].into()
    }
}

//这个样式给我写成狗了
//
// .style(|_theme, status| match status {
//                 iced::widget::scrollable::Status::Active {
//                     is_horizontal_scrollbar_disabled,
//                     is_vertical_scrollbar_disabled,
//                 } => iced::widget::scrollable::Style {
//                     container: iced::widget::container::Style {
//                         text_color: Some(iced::Color::from_rgb(1.0, 1.0, 1.0)),
//                         background: Some(iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         ))),
//                         border: iced::Border::default(),
//                         shadow: iced::Shadow::default(),
//                         snap: true,
//                     },
//                     vertical_rail: iced::widget::scrollable::Rail {
//                         background: Some(iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         ))),
//                         border: iced::Border::default(),
//                         scroller: iced::widget::scrollable::Scroller {
//                             background: iced::Background::Color(iced::Color::from_rgba(
//                                 0.0, 0.0, 0.0, 0.1,
//                             )),
//                             border: iced::Border::default(),
//                         },
//                     },
//                     horizontal_rail: iced::widget::scrollable::Rail {
//                         background: Some(iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         ))),
//                         border: iced::Border::default(),
//                         scroller: iced::widget::scrollable::Scroller {
//                             background: iced::Background::Color(iced::Color::from_rgba(
//                                 0.0, 0.0, 0.0, 0.1,
//                             )),
//                             border: iced::Border::default(),
//                         },
//                     },
//                     gap: Some(iced::Background::Color(iced::Color::from_rgba(
//                         0.0, 0.0, 0.0, 0.1,
//                     ))),
//                     auto_scroll: iced::widget::scrollable::AutoScroll {
//                         background: iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         )),
//                         border: iced::Border::default(),
//                         shadow: iced::Shadow::default(),
//                         icon: iced::Color::from_rgba(1.0, 1.0, 1.0, 1.0),
//                     },
//                 },
//                 iced::widget::scrollable::Status::Hovered() => iced::widget::scrollable::Style {
//                     container: iced::widget::container::Style {
//                         text_color: Some(iced::Color::from_rgb(1.0, 1.0, 1.0)),
//                         background: Some(iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         ))),
//                         border: iced::Border::default(),
//                         shadow: iced::Shadow::default(),
//                         snap: true,
//                     },
//                     vertical_rail: iced::widget::scrollable::Rail {
//                         background: Some(iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         ))),
//                         border: iced::Border::default(),
//                         scroller: iced::widget::scrollable::Scroller {
//                             background: iced::Background::Color(iced::Color::from_rgba(
//                                 0.0, 0.0, 0.0, 0.1,
//                             )),
//                             border: iced::Border::default(),
//                         },
//                     },
//                     horizontal_rail: iced::widget::scrollable::Rail {
//                         background: Some(iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         ))),
//                         border: iced::Border::default(),
//                         scroller: iced::widget::scrollable::Scroller {
//                             background: iced::Background::Color(iced::Color::from_rgba(
//                                 0.0, 0.0, 0.0, 0.1,
//                             )),
//                             border: iced::Border::default(),
//                         },
//                     },
//                     gap: Some(iced::Background::Color(iced::Color::from_rgba(
//                         0.0, 0.0, 0.0, 0.1,
//                     ))),
//                     auto_scroll: iced::widget::scrollable::AutoScroll {
//                         background: iced::Background::Color(iced::Color::from_rgba(
//                             0.0, 0.0, 0.0, 0.1,
//                         )),
//                         border: iced::Border::default(),
//                         shadow: iced::Shadow::default(),
//                         icon: iced::Color::from_rgba(1.0, 1.0, 1.0, 1.0),
//                     },
//                 },
//             });
//
