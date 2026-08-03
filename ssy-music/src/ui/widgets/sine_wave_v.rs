use iced::{
    Point,
    advanced::graphics::geometry::Frame,
    widget::canvas::{self, Path, Stroke},
};

#[derive(Debug, Clone)]
pub enum SineWaveMessage {
    SpectrumUpdated(Vec<f32>),
    PhaseUpdated(f32),
}

pub struct SineWave<'a> {
    spectrum: &'a [f32],
    phase: f32,
}

impl<'a> SineWave<'a> {
    pub fn new(spectrum: &'a [f32], phase: f32) -> Self {
        Self { spectrum, phase }
    }
}

impl<'a, Message> canvas::Program<Message> for SineWave<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry<iced::Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());
        let size = bounds.size();
        let mid_y = size.height / 2.0;

        // 计算 低频能量 (Bass)
        let bass_energy = if !self.spectrum.is_empty() {
            self.spectrum.iter().take(5).sum::<f32>() / 5.0
        } else {
            0.0
        };

        // 鼓点越强，波幅 (amplitude) 越大
        let amplitude = 15.0 + bass_energy * 100.0;

        let samples = 100;
        let num_waves = 3.0;
        let frequency = (2.0 * std::f32::consts::PI * num_waves) / size.width;

        let wave_path = Path::new(|builder| {
            let x_step = size.width / (samples - 1) as f32;

            let start_x = 0.0;
            let start_y = mid_y + amplitude * (frequency * start_x + self.phase).sin();
            builder.move_to(Point {
                x: start_x,
                y: start_y,
            });

            for i in 1..samples {
                let current_x = i as f32 * x_step;
                let current_y = mid_y + amplitude * (frequency * current_x + self.phase).sin();

                builder.line_to(Point {
                    x: current_x,
                    y: current_y,
                });
            }
        });

        frame.stroke(
            &wave_path,
            Stroke {
                style: canvas::Style::Solid(iced::Color::WHITE),
                width: 2.0,
                ..Stroke::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
