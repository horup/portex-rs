use ggsdk::{GGRunOptions, egui::{Color32, InputState, LayerId, Painter, Stroke, Key}};
use portex::World;

#[derive(Default)]
struct App {
    pub world: World,
    pub scale:i32,
}

impl App {
    pub fn draw_grid(&self, painter:&Painter) {
        // draw grid
        let color = Color32::DARK_GRAY;
        let w = painter.clip_rect().width() as i32;
        let h = painter.clip_rect().height() as i32;
        for y in (0..h).step_by(self.scale as usize) {
            painter.line_segment([(0.0, y as f32).into(), (w as f32, y as f32).into()], Stroke::new(1.0, color));
        }
        for x in (0..w).step_by(self.scale as usize) {
            painter.line_segment([(x as f32, 0.0).into(), (x as f32, h as f32).into()], Stroke::new(1.0, color));
        }
    }

    pub fn input(&mut self, input:&InputState) {
        if input.key_pressed(Key::Q) {
            self.scale = (self.scale * 2).min(128);
        }
        if input.key_pressed(Key::E) {
            self.scale = (self.scale / 2).max(4);
        }
    }
}

impl ggsdk::GGApp for App {
    fn init(&mut self, g: ggsdk::InitContext) {
        self.scale = 16;
    }

    fn update(&mut self, g: ggsdk::UpdateContext) {
        g.egui_ctx.input(|x|{
            self.input(x);
        });

        let painter = g.egui_ctx.layer_painter(LayerId::background());
        self.draw_grid(&painter);
    }
}

fn main() {
    ggsdk::GGEngine::run(
        App::default(),
        GGRunOptions {
            ..Default::default()
        },
    );
}
