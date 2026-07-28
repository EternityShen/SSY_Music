use std::{fs::File, time::Duration};

use rodio::{Decoder, MixerDeviceSink, Player, Source};

/// 音频播放器句柄
///     用它来播放音乐
pub struct PlayerHandle {
    // 虽然不用，必须在结构体内(保证它是活着的)，不然无法播放
    _device: MixerDeviceSink,
    // 播放器本体
    player: Player,
    // 备份，用于seek
    current_bytes: std::sync::Mutex<Option<Vec<u8>>>,
}

// 实现默认构造
impl Default for PlayerHandle {
    /// 构造一个默认的PlayerHandle句柄
    fn default() -> Self {
        let _device = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let player = Player::connect_new(_device.mixer());

        Self {
            _device,
            player,
            current_bytes: std::sync::Mutex::new(None),
        }
    }
}

impl PlayerHandle {
    /// 继续
    pub fn play(&self) {
        self.player.play();
    }

    /// 暂停
    pub fn pause(&self) {
        self.player.pause();
    }

    /// 时间跳转
    pub fn seek(&self, time: u64) {
        match self.player.try_seek(Duration::from_secs(time)) {
            Ok(_) => {}
            Err(_) => {
                // 调转失败直接从新播放
                if let Ok(guard) = self.current_bytes.lock() {
                    if let Some(data) = &*guard {
                        self.player.stop();

                        let cursor = std::io::Cursor::new(data.clone());
                        if let Ok(source) = rodio::Decoder::new(cursor) {
                            self.player.append(source);
                            let _ = self.player.try_seek(Duration::from_secs(time));
                            self.play();
                        }
                    }
                }
            }
        }
    }

    /// 本地路径播放 会返回一个Option 音乐时长
    ///     -> Option<u64>
    pub fn play_path(&self, path: String) -> Option<u64> {
        self.player.stop();
        let file = File::open(path).expect("Error1");
        let source = Decoder::try_from(file).expect("Error2");
        let duration = source.total_duration().map(|d| d.as_secs());
        self.player.append(source);
        duration
    }

    /// 字节数据播放
    pub fn play_form_bytes(&self, data: Vec<u8>) {
        self.player.stop();
        if let Ok(mut guard) = self.current_bytes.lock() {
            *guard = Some(data.clone());
        }
        let cursor = std::io::Cursor::new(data);
        match rodio::Decoder::new(cursor) {
            Ok(source) => {
                self.player.append(source);
                self.play();
            }
            Err(_e) => {
                todo!()
            }
        }
    }

    /// 获取当前播放到的时间
    ///     -> u64
    pub fn get_pos_time(&self) -> u64 {
        let pos = self.player.get_pos();
        pos.as_secs()
    }

    /// 播放器内是否为空(空就是没有音频在播放了)
    pub fn is_empty(&self) -> bool {
        self.player.empty()
    }
}
