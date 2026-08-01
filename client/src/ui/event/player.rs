/// 播放器事件
pub enum PlayerEvent {
    Play,
    Pause,
    Seek(u64),
    PlayBytes(Vec<u8>),
}
