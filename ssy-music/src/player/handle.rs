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
    current_path: String,

    // 记录producer
    producer: std::sync::Arc<std::sync::Mutex<ringbuf::HeapProd<f32>>>,

    // 计算器
    start_position: std::sync::Arc<std::sync::Mutex<u64>>,
    start_instant: std::sync::Arc<std::sync::Mutex<Option<std::time::Instant>>>,
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
            current_path: "OVO".to_string(),
            producer: producer_arc,
            start_position: std::sync::Arc::new(std::sync::Mutex::new(0)),
            start_instant: std::sync::Arc::new(std::sync::Mutex::new(None)),
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
        let seek_duration = Duration::from_secs(time);

        //优先尝试 Rodio 原生的 try_seek
        if self.player.try_seek(seek_duration).is_ok() {
            return;
        }

        let update_internal_time = || {
            if let Ok(mut pos) = self.start_position.lock() {
                *pos = time;
            }
            if let Ok(mut inst) = self.start_instant.lock() {
                *inst = Some(std::time::Instant::now());
            }
        };

        //如果 try_seek 失败，尝试从内存 Bytes 恢复
        let mut handled = false;
        if let Ok(guard) = self.current_bytes.lock() {
            if let Some(data) = &*guard {
                self.player.stop();

                let cursor = std::io::Cursor::new(data.clone());
                if let Ok(source) = rodio::Decoder::new(cursor) {
                    let skipped_source = source.skip_duration(seek_duration);

                    let visualizable_source = VisualizableSource {
                        input: skipped_source,
                        producer: std::sync::Arc::clone(&self.producer),
                    };

                    self.player.append(visualizable_source);
                    self.play();
                    update_internal_time();
                    handled = true;
                }
            }
        }

        //如果 current_bytes 是 None (说明是 play_path 播放的)，从 current_path 恢复！
        if !handled {
            self.player.stop();
            if let Ok(file) = std::fs::File::open(&self.current_path) {
                if let Ok(source) = rodio::Decoder::try_from(file) {
                    let skipped_source = source.skip_duration(seek_duration);

                    let visualizable_source = VisualizableSource {
                        input: skipped_source,
                        producer: std::sync::Arc::clone(&self.producer),
                    };

                    self.player.append(visualizable_source);
                    self.play();
                    update_internal_time();
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
    pub fn play_path(&mut self, path: String) -> Option<u64> {
        self.player.stop();
        let file = File::open(&path).expect("Error1");
        self.current_path = path;
        let source = Decoder::try_from(file).expect("Error2");
        let duration = source.total_duration().map(|d| d.as_secs());
        let visualizablesource = VisualizableSource {
            input: source,
            producer: std::sync::Arc::clone(&self.producer),
        };
        self.player.append(visualizablesource);
        self.play();
        if let Ok(mut pos) = self.start_position.lock() {
            *pos = 0;
        }
        if let Ok(mut inst) = self.start_instant.lock() {
            *inst = Some(std::time::Instant::now());
        }
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
                if let Ok(mut pos) = self.start_position.lock() {
                    *pos = 0;
                }
                if let Ok(mut inst) = self.start_instant.lock() {
                    *inst = Some(std::time::Instant::now());
                }
            }
            Err(_e) => {
                todo!()
            }
        }
    }

    /// 获取当前播放到的时间
    ///     -> u64
    pub fn get_pos_time(&self) -> u64 {
        let base_pos = self.start_position.lock().map(|p| *p).unwrap_or(0);

        if let Ok(inst_guard) = self.start_instant.lock() {
            if let Some(start_time) = *inst_guard {
                // 如果正在播放，加上从上次 Seek/播放 到现在所经过的时间
                if !self.player.is_paused() {
                    let elapsed = start_time.elapsed().as_secs();
                    return base_pos + elapsed;
                }
            }
        }

        base_pos
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
