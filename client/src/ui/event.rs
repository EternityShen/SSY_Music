pub enum PlayerEvent {
    Ready(iced::futures::channel::mpsc::Sender<PlayerHandleEvent>),
    MusicTime(u64),
    Next,
}

pub enum PlayerHandleEvent {
    Play,
    Pause,
    Seek(u64),
    PlayBytes(Vec<u8>),
}
