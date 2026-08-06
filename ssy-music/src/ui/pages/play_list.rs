use crate::ui::widgets;

pub struct PlayListPage {
    items: Vec<widgets::play_list_item::PlayListItem>,
    ids: Vec<u64>,
}

pub enum PlayListEvent {
    Play(u64),
    Delete(u64),
}

pub enum PlayListPageMessage {
    OnPress(widgets::play_list_item::PlayListItemMessage),
}

impl PlayListPage {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            ids: Vec::new(),
        }
    }

    pub fn update(&mut self, message: PlayListPageMessage) -> Option<PlayListEvent> {
        match message {
            PlayListPageMessage::OnPress(message) => match message {
                widgets::play_list_item::PlayListItemMessage::Delete(id) => {
                    if let Some(pos) = self.items.iter().position(|item| item.id == id) {
                        self.items.remove(pos);
                    }
                    Some(PlayListEvent::Delete(id))
                }
                widgets::play_list_item::PlayListItemMessage::OnPress(id) => {
                    Some(PlayListEvent::Play(id))
                }
            },
        }
    }

    pub fn add_item(&mut self, data: (crate::api::data::Song, Vec<u8>)) {
        for id in &self.ids {
            if id == &data.0.id {
                return;
            }
        }
        self.ids.push(data.0.id);
        let item = widgets::play_list_item::PlayListItem::new(data.0, data.1);
        self.items.push(item);
    }

    pub fn view(&self) -> iced::Element<'_, PlayListPageMessage> {
        let list_content =
            self.items
                .iter()
                .fold(iced::widget::column![].spacing(6), |col, item| {
                    let item_element = item.view().map(PlayListPageMessage::OnPress);
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

        scrollable_list.into()
    }
}

impl Default for PlayListPage {
    fn default() -> Self {
        Self::new()
    }
}
