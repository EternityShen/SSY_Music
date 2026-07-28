/// 音乐进度条
pub struct MusicSlider {
    slider_value: f32,
    all_value: f32,
}

/// 消息
#[derive(Debug, Clone)]
pub enum MusicSliderMessage {
    Seek(f32),
    SetAllValue(f32),
}

impl MusicSlider {
    /// 创建
    pub fn new() -> Self {
        Self {
            slider_value: 0.0,
            all_value: 100.0,
        }
    }

    /// 更新
    pub fn update(&mut self, message: MusicSliderMessage) {
        match message {
            MusicSliderMessage::Seek(value) => {
                self.slider_value = value;
            }
            MusicSliderMessage::SetAllValue(value) => {
                self.all_value = value;
            }
        }
    }

    /// 渲染
    pub fn view(&self) -> iced::Element<'_, MusicSliderMessage> {
        // 进度条
        let slider = iced::widget::Slider::new(0.0..=self.all_value, self.slider_value, |value| {
            MusicSliderMessage::Seek(value)
        })
        .step(1.0_f32)
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

        // 时长信息
        let time_info = iced::widget::row![
            iced::widget::text(all_time)
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

impl Default for MusicSlider {
    /// 默认构造,应付语法服务器
    fn default() -> Self {
        Self::new()
    }
}
