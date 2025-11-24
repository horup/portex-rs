use std::mem::take;

use ggsdk::{GGRunOptions, egui::{Align2, Color32, FontId, InputState, Key, LayerId, Painter, Pos2, Stroke}};
use portex::{LinesBuilder, World};

#[derive(Default)]
struct App {
    pub world: World,
    pub scale:i32,
    pub line_builder:LinesBuilder,
    pub pointer_world:(i32, i32),
    pub pointer_screen:Pos2,
    pub offset:(i32, i32),
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
        let offset_speed = 16.0;
        if input.key_pressed(Key::E) {
            self.scale = (self.scale / 2).max(4);
        }
        if input.key_pressed(Key::W) {
            self.offset.1 += offset_speed as i32;
        }
        if input.key_pressed(Key::S) {
            self.offset.1 -= offset_speed as i32;
        }
        if input.key_pressed(Key::A) {
            self.offset.0 += offset_speed as i32;
        }
        if input.key_pressed(Key::D) {
            self.offset.0 -= offset_speed as i32;
        }
        

        if let Some(mut pointer_pos) = input.pointer.latest_pos() {
            self.pointer_screen = pointer_pos;
            self.pointer_world = self.screen_to_world(pointer_pos);
        }

        if input.pointer.any_pressed() {
            let p = self.pointer_world;
            self.line_builder.push_vertex(p.0, p.1);
        }

        if input.key_pressed(Key::Space) {
            let lines_builder = take(&mut self.line_builder);
            lines_builder.build(&mut self.world);
        }

        if input.key_pressed(Key::Backspace) {
            self.line_builder.pop_vertex();
        }
    }


    pub fn draw_lines(&mut self, painter:&Painter) {
        for (_, line) in self.world.lines_iter() {
            let v1 = self.world.vertex(line.p1).unwrap();
            let v2 = self.world.vertex(line.p2).unwrap();
            let p1 = self.world_to_screen((v1.x, v1.y));
            let p2 = self.world_to_screen((v2.x, v2.y));
            painter.line_segment([p1, p2], Stroke::new(1.0, Color32::WHITE));
        }
    }   

    pub fn draw_line_builder(&mut self, painter:&Painter) {
        let vertices = &self.line_builder.vertices;
        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let next_i = (i + 1) % vertices.len();
            let v2 = vertices[next_i];
            let p1 = self.world_to_screen((v1.x, v1.y));
            let p2 = self.world_to_screen((v2.x, v2.y));
            painter.line_segment([p1, p2], Stroke::new(1.0, Color32::YELLOW));
        }
    }
    pub fn draw_ui(&mut self, painter:&Painter) {
        let p = self.pointer_world;
        painter.text(Pos2::new(16.0, 16.0), Align2::LEFT_CENTER, format!("Pointer: {}, {}", p.0, p.1), FontId::default(), Color32::WHITE);
        painter.text(Pos2::new(16.0, 32.0), Align2::LEFT_CENTER, format!("Offset: {}, {}", self.offset.0, self.offset.1), FontId::default(), Color32::WHITE);

    }

    pub fn screen_to_world(&self, screen_pos:Pos2) -> (i32, i32) {
        let x = (screen_pos.x as i32 - self.offset.0) / self.scale;
        let y = (screen_pos.y as i32 - self.offset.1) / self.scale;
        (x, y)
    }

    pub fn world_to_screen(&self, world_pos:(i32, i32)) -> Pos2 {
        let x = (world_pos.0 * self.scale) + self.offset.0;
        let y = (world_pos.1 * self.scale) + self.offset.1;
        Pos2::new(x as f32, y as f32)
    }
}

impl ggsdk::GGApp for App {
    fn init(&mut self, g: ggsdk::InitContext) {
        self.scale = 1;
    }

    fn update(&mut self, g: ggsdk::UpdateContext) {
        g.egui_ctx.input(|x|{
            self.input(x);
        });

        let painter = g.egui_ctx.layer_painter(LayerId::background());
        self.draw_grid(&painter);
        self.draw_lines(&painter);
        self.draw_line_builder(&painter);
        self.draw_ui(&painter);
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
