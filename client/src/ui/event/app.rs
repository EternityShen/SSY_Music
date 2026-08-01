/// App事件
pub enum AppEvent {
    Ready(iced::futures::channel::mpsc::Sender<super::player::PlayerEvent>),
    PlayTime(u64),
    Next,
}
