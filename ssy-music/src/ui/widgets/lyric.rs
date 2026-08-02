#[derive(Clone)]
pub struct LyricData {
    pub str: String,
    pub time: f32,
}

impl Default for LyricData {
    fn default() -> Self {
        Self {
            str: "".to_string(),
            time: 0.0,
        }
    }
}

pub enum LyricMessage {
    SetLyrics(String),
    SetTime(u64),
}

pub struct Lyric {
    time: u64,
    lyrics: Vec<LyricData>,
}

fn parse_lrc_time(time_str: &str) -> Option<f32> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let minutes: f32 = parts[0].parse().ok()?;
    let seconds: f32 = parts[1].parse().ok()?;

    Some(minutes * 60.0 + seconds)
}

pub fn new_lyrics(text: String) -> Vec<LyricData> {
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
                lyrics.push(LyricData {
                    str: lyric_str.trim().to_string(),
                    time: total_seconds,
                });
            }
        }
    }

    lyrics
}

impl Lyric {
    pub fn new() -> Self {
        let lyrics = vec![
            LyricData {
                str: "人生路漫漫,每个人都有属于自己的浪漫".to_string(),
                time: 0.0,
            },
            LyricData {
                str: "多么想让这独属于自己的浪漫成为永恒".to_string(),
                time: 1.0,
            },
        ];

        Self { time: 0, lyrics }
    }

    pub fn update(&mut self, message: LyricMessage) {
        match message {
            LyricMessage::SetLyrics(data) => {
                let lyrics = new_lyrics(data);
                self.lyrics = lyrics;
            }
            LyricMessage::SetTime(time) => {
                self.time = time;
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, LyricMessage> {
        let index = self
            .lyrics
            .iter()
            .position(|line| line.time > self.time as f32)
            .map(|i| i.saturating_sub(1))
            .unwrap_or_else(|| {
                if self.lyrics.is_empty() {
                    0
                } else {
                    self.lyrics.len() - 1
                }
            });

        let now_lyric_data = self.lyrics.get(index).cloned().unwrap_or_default();
        let next_lyric_data = self.lyrics.get(index + 1).cloned().unwrap_or_default();

        let now_lyric = iced::widget::container(
            iced::widget::text(now_lyric_data.str)
                .size(35)
                .color(iced::Color::from_rgb(0.9, 0.6, 0.6)),
        )
        .width(iced::Length::Fill)
        .center_x(iced::Length::Fill);
        let next_lyric = iced::widget::container(
            iced::widget::text(next_lyric_data.str)
                .size(20)
                .color(iced::Color::from_rgb(0.9, 0.6, 0.6)),
        )
        .width(iced::Length::Fill)
        .center_x(iced::Length::Fill);

        iced::widget::column![now_lyric, next_lyric]
            .spacing(10)
            .into()
    }
}

impl Default for Lyric {
    fn default() -> Self {
        Self::new()
    }
}
