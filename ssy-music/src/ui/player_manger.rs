use crate::api;
use crate::ui::event;

/// Player的管理者
pub struct PlayerManger {
    pub playersender:
        Option<iced::futures::channel::mpsc::Sender<super::event::player::PlayerEvent>>,
    pub load_data: api::load::LoadDate,
    pub playing_id: u64,
    pub volume: f32,
    pub list_num: u64,
    pub play_time: u64,
    pub is_play: bool,
}

impl PlayerManger {
    pub fn new(db_path: &str, lyrics_path: &str) -> Self {
        let load_data = api::load::LoadDate::load_data_from_toml(db_path, lyrics_path);
        Self {
            playersender: None,
            load_data,
            playing_id: 0,
            volume: 1.0,
            list_num: 0,
            play_time: 0,
            is_play: false,
        }
    }

    /// 设置 sender
    pub fn set_sender(
        &mut self,
        sender: iced::futures::channel::mpsc::Sender<event::player::PlayerEvent>,
    ) {
        self.playersender = Some(sender)
    }

    /// 播放字节数据
    pub fn play_bytes(&mut self, data: Vec<u8>) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::PlayBytes(data));
        self.is_play = true;
    }

    /// 播放指定路径的音频
    pub fn play_path(&mut self, path: String) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::PlayPath(path));
        self.is_play = true;
    }

    /// 下一首的id
    pub fn next_id(&mut self) -> u64 {
        if self.playing_id >= self.list_num {
            self.playing_id = 0;
        } else {
            self.playing_id += 1;
        }
        self.playing_id
    }

    /// 播放
    pub fn play(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Play);
        self.is_play = true;
    }

    /// 暂停
    pub fn pause(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Pause);
        self.is_play = false;
    }

    /// 跳转
    pub fn seek(&self, time: u64) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Seek(time));
    }

    /// 快进5s
    pub fn seek_add_5(&self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Seek(self.play_time + 5));
    }

    /// 倒退5s
    pub fn seek_subtract_5(&self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::Seek(
                self.play_time.saturating_sub(5),
            ));
    }

    /// 音量+5%
    pub fn volume_add_5(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::SetVolume(
                (self.volume + 0.05).clamp(0.0, 1.0),
            ));

        self.volume = (self.volume + 0.05).clamp(0.0, 1.0);
    }

    /// 音量-5%
    pub fn volume_subtract_5(&mut self) {
        let _ = self
            .playersender
            .clone()
            // 服务会在软件启动时启动，sender会在服务启动时set到manger(所以，不可能炸，当然，不排除玄学)
            .unwrap()
            .try_send(super::event::player::PlayerEvent::SetVolume(
                (self.volume - 0.05).clamp(0.0, 1.0),
            ));

        self.volume = (self.volume - 0.05).clamp(0.0, 1.0);
    }
}
