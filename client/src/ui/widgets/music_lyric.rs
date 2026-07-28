#[derive(Debug, Clone)]
pub struct Lyric {
    time: f32,
    str: String,
}

/// 歌词
pub struct MusicLyric {
    now_time: f32,
    lyrics: Vec<Lyric>,
}

/// 消息
pub enum MusicLyricMessage {
    GetLyrics((std::sync::Arc<crate::api::request::MusicClient>, u64)),
    Set(Vec<Lyric>),
    Time(f32),
}

/// 时间转换
fn parse_lrc_time(time_str: &str) -> Option<f32> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let minutes: f32 = parts[0].parse().ok()?;
    let seconds: f32 = parts[1].parse().ok()?;

    Some(minutes * 60.0 + seconds)
}

/// 歌词解析
pub async fn get_fmt_lyrics(
    api: std::sync::Arc<crate::api::request::MusicClient>,
    id: u64,
) -> Vec<Lyric> {
    let option = api.fetch_lyrics(id).await;
    match option {
        Some(text) => {
            let mut lyrics = Vec::new();
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if line.starts_with('[')
                    && line.len() >= 10
                    && let Some(end_bracket_idx) = line.find(']')
                {
                    let time_str = &line[1..end_bracket_idx];
                    let lyric_str = &line[end_bracket_idx + 1..];

                    if let Some(total_seconds) = parse_lrc_time(time_str) {
                        lyrics.push(Lyric {
                            str: lyric_str.trim().to_string(),
                            time: total_seconds,
                        });
                    }
                }
            }

            lyrics
        }
        None => {
            vec![
                Lyric {
                    time: 0.0,
                    str: "人生路漫漫每个人都有自己的浪漫".to_string(),
                },
                Lyric {
                    time: 1.0,
                    str: "多么想让这独属于自己的浪漫成为永恒".to_string(),
                },
            ]
        }
    }
}

impl MusicLyric {
    /// 创建
    pub fn new() -> Self {
        let lyrics = vec![
            Lyric {
                time: 0.0,
                str: "人生路漫漫每个人都有自己的浪漫".to_string(),
            },
            Lyric {
                time: 1.0,
                str: "多么想让这独属于自己的浪漫成为永恒".to_string(),
            },
        ];
        Self {
            now_time: 0.0,
            lyrics,
        }
    }

    /// 更新
    pub fn update(&mut self, message: MusicLyricMessage) -> iced::Task<MusicLyricMessage> {
        match message {
            MusicLyricMessage::GetLyrics(data) => {
                return iced::Task::perform(get_fmt_lyrics(data.0, data.1), MusicLyricMessage::Set);
            }
            MusicLyricMessage::Set(lyrics) => {
                self.lyrics = lyrics;
            }
            MusicLyricMessage::Time(time) => {
                self.now_time = time;
            }
        };
        iced::Task::none()
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, MusicLyricMessage> {
        let value = self
            .lyrics
            .clone()
            .iter()
            .position(|line| line.time > self.now_time)
            .map(|l| l.saturating_sub(1))
            .unwrap_or_else(|| {
                if self.lyrics.is_empty() {
                    0
                } else {
                    self.lyrics.len() - 1
                }
            });

        let str = if value == self.lyrics.len() - 1 {
            String::from("")
        } else {
            self.lyrics[value + 1].clone().str
        };

        let now = iced::widget::text(self.lyrics[value].clone().str)
            .size(25)
            .color(iced::Color::from_rgb(0.9, 0.6, 0.6));

        let next = iced::widget::text(str)
            .size(25)
            .color(iced::Color::from_rgb(0.9, 0.6, 0.6));

        let now_lyric = iced::widget::row![now, iced::widget::space::horizontal()];

        let next_lyric = iced::widget::row![iced::widget::space::horizontal(), next];

        iced::widget::column![now_lyric, next_lyric]
            .height(70)
            .into()
    }
}

impl Default for MusicLyric {
    /// 应付语法服务器
    fn default() -> Self {
        Self::new()
    }
}
