/// 音乐进度条
pub struct ProgressBar {
    slider_value: f32,
    all_value: f32,
    volume: f32,
    is_on: bool,
}

/// 消息
#[derive(Debug, Clone)]
pub enum ProgressBarMessage {
    Seek(f32),
    OnSeek(f32),
    SetAllValue(f32),
    SetVolume(f32),
    OnRelease,
}

impl ProgressBar {
    /// 创建
    pub fn new() -> Self {
        Self {
            slider_value: 0.0,
            all_value: 100.0,
            volume: 1.0,
            is_on: false,
        }
    }

    /// 更新
    pub fn update(&mut self, message: ProgressBarMessage) -> Option<f32> {
        match message {
            ProgressBarMessage::Seek(value) => {
                if self.is_on {
                    return None;
                }
                self.slider_value = value;
                None
            }
            ProgressBarMessage::OnSeek(value) => {
                self.is_on = true;
                self.slider_value = value;
                None
            }
            ProgressBarMessage::SetAllValue(value) => {
                self.all_value = value;
                None
            }
            ProgressBarMessage::SetVolume(value) => {
                self.volume = value;
                None
            }
            ProgressBarMessage::OnRelease => {
                self.is_on = false;
                return Some(self.slider_value);
            }
        }
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, ProgressBarMessage> {
        // 进度条
        let slider = iced::widget::Slider::new(
            0.0..=self.all_value,
            self.slider_value,
            ProgressBarMessage::OnSeek,
        )
        .step(1.0_f32)
        .on_release(ProgressBarMessage::OnRelease)
        .default(10.0_f32)
        .width(iced::Length::Fixed(300.0))
        .style(|_theme, status| match status {
            iced::widget::slider::Status::Active => iced::widget::slider::Style {
                rail: iced::widget::slider::Rail {
                    backgrounds: (
                        iced::Background::Color(iced::Color::from_rgb(0.9, 0.5, 0.6)),
                        iced::Background::Color(iced::Color::from_rgb(0.7, 0.5, 0.6)),
                    ),
                    width: 2.0,
                    border: iced::Border::default(),
                },
                handle: iced::widget::slider::Handle {
                    shape: iced::widget::slider::HandleShape::Rectangle {
                        width: 10,
                        border_radius: 6.0.into(),
                    },
                    background: iced::Background::Color(iced::Color::from_rgb(0.9, 0.5, 0.6)),
                    border_width: 0.0,
                    border_color: iced::Color::from_rgb(0.6, 0.6, 0.6),
                },
            },
            iced::widget::slider::Status::Hovered | iced::widget::slider::Status::Dragged => {
                iced::widget::slider::Style {
                    rail: iced::widget::slider::Rail {
                        backgrounds: (
                            iced::Background::Color(iced::Color::from_rgb(0.9, 0.7, 0.7)),
                            iced::Background::Color(iced::Color::from_rgb(0.7, 0.5, 0.6)),
                        ),
                        width: 2.0,
                        border: iced::Border::default(),
                    },
                    handle: iced::widget::slider::Handle {
                        shape: iced::widget::slider::HandleShape::Rectangle {
                            width: 12,
                            border_radius: 5.0.into(),
                        },
                        background: iced::Background::Color(iced::Color::from_rgb(0.9, 0.6, 0.6)),
                        border_width: 0.0,
                        border_color: iced::Color::from_rgb(0.6, 0.6, 0.6),
                    },
                }
            }
        });

        let all_time = format!(
            "{}分{}秒",
            (self.all_value - (self.all_value % 60.0)) / 60.0,
            (self.all_value % 60.0) as u32
        );

        let slider_time = format!(
            "{}分{}秒",
            (self.slider_value - (self.slider_value % 60.0)) / 60.0,
            (self.slider_value % 60.0) as u32
        );

        let volume = format!("音量:{}%", (self.volume * 100.0) as u32);

        // 时长音量信息
        let time_info = iced::widget::row![
            iced::widget::text(all_time)
                .size(10)
                .color(iced::Color::from_rgb(0.9, 0.6, 0.6)),
            iced::widget::space::horizontal(),
            iced::widget::text(volume)
                .size(10)
                .color(iced::Color::from_rgb(0.9, 0.6, 0.6)),
            iced::widget::space::horizontal(),
            iced::widget::text(slider_time)
                .size(10)
                .color(iced::Color::from_rgb(0.9, 0.6, 0.6)),
        ]
        .width(300);

        iced::widget::column![slider, time_info].spacing(10).into()
    }
}

impl Default for ProgressBar {
    /// 默认构造,应付语法服务器
    fn default() -> Self {
        Self::new()
    }
}
