use eframe::egui;
use egui::{Color32, RichText, TextEdit};
use std::{
    env::current_dir,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};
struct DirectoryApp {
    file_content: String,
    current_dir: PathBuf,
    error_message: Option<String>,
}
impl DirectoryApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_content: String::new(),
            current_dir: current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            error_message: None,
        }
    }

    fn set_error(&mut self, error: impl ToString) {
        self.error_message = Some(error.to_string());
    }

    fn load_file(&mut self, file_path: PathBuf) {
        match read_to_string(file_path) {
            Ok(content) => {
                self.file_content = content;
                self.error_message = None;
            }
            Err(e) => self.set_error(e),
        }
    }
}

impl eframe::App for DirectoryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("File browser")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("⬆").size(14.0)).clicked()
                        && self.current_dir.parent().is_some()
                    {
                        self.current_dir.pop();
                    }

                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        let path_text = self.current_dir.to_string_lossy().to_string();
                        ui.label(RichText::new(path_text).size(11.0));
                    });
                });

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Ok(read_dir) = read_dir(&self.current_dir) {
                        for entry in read_dir.flatten() {
                            if let Ok(metadata) = entry.metadata() {
                                if let Ok(name) = entry.file_name().into_string() {
                                    let is_dir = metadata.is_dir();
                                    let icon = if is_dir { "📁 " } else { "📄 " };

                                    let color = if ui.visuals().dark_mode {
                                        if is_dir {
                                            Color32::from_rgb(110, 166, 255)
                                        } else {
                                            Color32::from_rgb(255, 210, 120)
                                        }
                                    } else {
                                        if is_dir {
                                            Color32::from_rgb(30, 100, 200)
                                        } else {
                                            Color32::from_rgb(180, 140, 0)
                                        }
                                    };

                                    let response = ui.add(
                                        egui::Button::new(
                                            RichText::new(format!("{}{}", icon, name))
                                                .color(color)
                                                .size(13.0),
                                        )
                                        .fill(Color32::TRANSPARENT)
                                        .min_size(egui::vec2(ui.available_width(), 0.0)),
                                    );

                                    if response.clicked() {
                                        if is_dir {
                                            self.current_dir.push(name);
                                        } else {
                                            let file_path = self.current_dir.join(name);
                                            self.load_file(file_path);
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(error) = &self.error_message {
                ui.colored_label(Color32::RED, error);
            } else if !self.file_content.is_empty() {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        TextEdit::multiline(&mut self.file_content)
                            .desired_width(f32::INFINITY)
                            .desired_rows(30)
                            .code_editor(),
                    );
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Select a file to view its contents");
                });
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "File explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(DirectoryApp::new(cc)))),
    );
}
