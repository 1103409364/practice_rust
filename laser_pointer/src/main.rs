#[derive(Default)]
struct NothingApp {
    number: i32,
    text: String,
    code: String,
}

impl NothingApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            number: 0,
            text: String::from("Put some text in here!"),
            code: String::from(r#"fn main() { println!("Hello, world!"); }"#),
        }
    }
}

impl eframe::App for NothingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Counter up").clicked() {
                self.number += 1
            }
            if ui.button("Counter down").clicked() {
                self.number -= 1
            }
            ui.label(format!("The counter is: {}", self.number));
            ui.text_edit_multiline(&mut self.text);
            ui.code_editor(&mut self.code);
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(NothingApp::new(cc)))),
    );
}
