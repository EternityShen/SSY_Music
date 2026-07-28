/// 不知道怎么讲(appwindow 的订阅中用的 Event)
pub enum PlayerEvent {
    Ready(iced::futures::channel::mpsc::Sender<PlayerHandleEvent>),
    MusicTime(u64),
    Next,
}

/// PlayerHandle的 Event
pub enum PlayerHandleEvent {
    Play,
    Pause,
    Seek(u64),
    PlayBytes(Vec<u8>),
}
