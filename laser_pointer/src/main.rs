use eframe::egui;
use egui::{Color32, Pos2, Rect, Sense, Vec2};
#[derive(Default, Clone, Copy)]
struct LaserPointer {
    x: f32,
    y: f32,
    speed: Speed,
    imaginary_target: Pos2,
}
#[derive(Clone, Copy, Default)]
enum Speed {
    #[default]
    Still,
    Slow,
    Fast,
    CrazyFast,
}
impl From<LaserPointer> for Pos2 {
    fn from(pointer: LaserPointer) -> Self {
        Pos2 {
            x: pointer.x,
            y: pointer.y,
        }
    }
}
impl LaserPointer {
    fn random_movement(&mut self, amount: f32) {
        if fastrand::bool() {
            self.x += fastrand::f32() * amount;
        } else {
            self.x -= fastrand::f32() * amount;
        }
        if fastrand::bool() {
            self.y += fastrand::f32() * amount;
        } else {
            self.y -= fastrand::f32() * amount;
        }
    }
    fn try_change_speed(&mut self) {
        use Speed::*;
        if fastrand::f32() > 0.98 {
            self.speed = match fastrand::u8(0..3) {
                0 => Still,
                1 => Slow,
                2 => Fast,
                _ => CrazyFast,
            }
        }
    }
    fn try_change_target(&mut self, rect: Rect) {
        let bottom_right = rect.max;
        if fastrand::f32() > 0.98 {
            self.imaginary_target = Pos2 {
                x: fastrand::f32() * bottom_right.x,
                y: fastrand::f32() * bottom_right.y,
            }
        }
    }
    fn change_speed(&self) -> f32 {
        match self.speed {
            Speed::Still => 0.0,
            Speed::Slow => 0.05,
            Speed::Fast => 0.1,
            Speed::CrazyFast => 0.3,
        }
    }
    fn move_self(&mut self) {
        let x_from_target = self.imaginary_target.x - self.x;
        let y_from_target = self.imaginary_target.y - self.y;
        self.x += fastrand::f32() * x_from_target * self.change_speed();
        self.y += fastrand::f32() * y_from_target * self.change_speed();
    }
}
impl LaserPointer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            x: 50.0,
            y: 50.0,
            speed: Speed::default(),
            imaginary_target: Pos2 { x: 50.0, y: 50.0 },
        }
    }
}
impl eframe::App for LaserPointer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ctx.screen_rect();

            self.try_change_speed();
            self.try_change_target(rect);
            self.move_self();
            let screen_size = Vec2 {
                x: rect.width(),
                y: rect.height(),
            };
            let (_, painter) = ui.allocate_painter(screen_size, Sense::hover());
            let LaserPointer { x, y, .. } = self;
            let Pos2 { x: x2, y: y2 } = ctx.pointer_hover_pos().unwrap_or_default();
            if (*x - x2).abs() < 20.0 && (*y - y2).abs() < 20.0 {
                self.random_movement(50.0);
            }
            painter.circle_filled(Pos2::from(*self), 20.0, Color32::RED);
        });
    }
}
fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Awesome laser pointer",
        native_options,
        Box::new(|cc| Ok(Box::new(LaserPointer::new(cc)))),
    );
}
