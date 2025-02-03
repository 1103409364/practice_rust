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
}
impl DirectoryApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_content: String::new(),
            current_dir: current_dir().unwrap(),
        }
    }
}

impl eframe::App for DirectoryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button(" .. ").clicked() {
                self.current_dir.pop();
            }
            if let Ok(read_dir) = read_dir(&self.current_dir) {
                for entry in read_dir.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            if let Ok(dir_name) = entry.file_name().into_string() {
                                if ui
                                    .button(RichText::new(&dir_name).color(Color32::GRAY))
                                    .clicked()
                                {
                                    self.current_dir.push(&dir_name);
                                }
                            }
                        } else if metadata.is_file() {
                            if let Ok(file_name) = entry.file_name().into_string() {
                                if ui
                                    .button(RichText::new(&file_name).color(Color32::GOLD))
                                    .clicked()
                                {
                                    if let Some(current_dir) = self.current_dir.to_str() {
                                        let file_loc: PathBuf =
                                            [current_dir, &file_name].iter().collect();
                                        let content = read_to_string(file_loc)
                                            .unwrap_or_else(|e| e.to_string());
                                        self.file_content = content;
                                    }
                                }
                            }
                        } else {
                            ui.label(format!("{:?}", metadata.file_type()));
                        }
                    }
                }
            }
        });
        let width = ctx.screen_rect().max.x / 2.0;
        if !self.file_content.is_empty() {
            egui::SidePanel::right("Text viewer")
                .min_width(width)
                .show(ctx, |ui| {
                    ui.add(TextEdit::code_editor(
                        TextEdit::multiline(&mut self.file_content).desired_width(width),
                    ));
                });
        }
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
