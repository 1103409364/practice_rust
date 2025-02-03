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
                    if ui.button("⬆ Up").clicked() && self.current_dir.parent().is_some() {
                        self.current_dir.pop();
                    }
                    ui.label(self.current_dir.to_string_lossy().to_string());
                });

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Ok(read_dir) = read_dir(&self.current_dir) {
                        for entry in read_dir.flatten() {
                            if let Ok(metadata) = entry.metadata() {
                                if let Ok(name) = entry.file_name().into_string() {
                                    let is_dir = metadata.is_dir();
                                    let icon = if is_dir { "📁 " } else { "📄 " };
                                    let color = if is_dir { Color32::LIGHT_BLUE } else { Color32::GOLD };
                                    
                                    if ui.button(RichText::new(format!("{}{}", icon, name))
                                        .color(color))
                                        .clicked() 
                                    {
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
                            .code_editor()
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
