use std::{fs::File, time::Duration};

use cpal::Sample;
use ringbuf::traits::Producer;
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
    // 记录producer
    producer: std::sync::Arc<std::sync::Mutex<ringbuf::HeapProd<f32>>>,
}

impl PlayerHandle {
    /// 创建
    pub fn new(producer_arc: std::sync::Arc<std::sync::Mutex<ringbuf::HeapProd<f32>>>) -> Self {
        let _device = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let player = Player::connect_new(_device.mixer());

        Self {
            _device,
            player,
            current_bytes: std::sync::Mutex::new(None),
            producer: producer_arc,
        }
    }

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

    /// 设置音量
    pub fn set_volume(&self, value: f32) {
        self.player.set_volume(value);
    }

    /// 获取当前音量
    pub fn volume(&self) -> f32 {
        self.player.volume()
    }

    /// 本地路径播放 会返回一个Option 音乐时长
    ///     -> Option<u64>
    pub fn play_path(&self, path: String) -> Option<u64> {
        self.player.stop();
        let file = File::open(path).expect("Error1");
        let source = Decoder::try_from(file).expect("Error2");
        let duration = source.total_duration().map(|d| d.as_secs());
        let visualizablesource = VisualizableSource {
            input: source,
            producer: std::sync::Arc::clone(&self.producer),
        };
        self.player.append(visualizablesource);
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
                let visualizablesource = VisualizableSource {
                    input: source,
                    producer: std::sync::Arc::clone(&self.producer),
                };
                self.player.append(visualizablesource);
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

/// 音频采样拦截器
/// 自动使用 dasp_sample 将任意 PCM 采样 (i16/u16/f32等) 转换为标准的 f32 并推入环形缓冲区
pub struct VisualizableSource<S> {
    pub input: S,
    pub producer: std::sync::Arc<std::sync::Mutex<ringbuf::HeapProd<f32>>>,
}

impl<S> Iterator for VisualizableSource<S>
where
    S: Iterator,
    S::Item: Sample + dasp_sample::ToSample<f32>, // 加上 ToSample<f32> 约束
{
    type Item = S::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // 获取原始采样
        let sample = self.input.next()?;

        // 使用 dasp_sample 的 .to_sample::<f32>() 安全转换为 f32 (-1.0 ~ 1.0)
        let sample_f32: f32 = sample.to_sample();

        // 非阻塞写入 FFT 环形缓冲区
        if let Ok(mut producer) = self.producer.try_lock() {
            let _ = producer.try_push(sample_f32);
        }

        // 返回原始采样供音频线程输出
        Some(sample)
    }
}

impl<S> Source for VisualizableSource<S>
where
    S: Source,
    S::Item: Sample + dasp_sample::ToSample<f32>,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.input.current_span_len()
    }

    #[inline]
    fn channels(&self) -> rodio::ChannelCount {
        self.input.channels()
    }

    #[inline]
    fn sample_rate(&self) -> rodio::SampleRate {
        self.input.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }
}
