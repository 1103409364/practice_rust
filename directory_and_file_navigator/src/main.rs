use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use egui::{Color32, RichText, TextEdit};
use std::{
    env::current_dir,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

/// 目录浏览应用的主要结构
struct DirectoryApp {
    file_content: String,          // 当前打开文件的内容
    current_dir: PathBuf,          // 当前浏览的目录路径
    error_message: Option<String>, // 错误信息，如果有的话
}

impl DirectoryApp {
    /// 创建新的应用实例
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 设置字体支持中文字符，字体文件在 /fonts/SmileySans-Oblique.ttf
        let mut fonts = FontDefinitions::default();
        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "SmileySans-Oblique".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                FontData::from_static(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/fonts/SmileySans-Oblique.otf"
                ))),
            ),
        );

        // Put my font first (highest priority):
        // fonts
        //     .families
        //     .get_mut(&FontFamily::Proportional)
        //     .unwrap()
        //     .insert(0, "SmileySans-Oblique".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("SmileySans-Oblique".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        // 初始化应用实例
        Self {
            file_content: String::new(),
            current_dir: current_dir().unwrap_or_else(|_| PathBuf::from(".")), // 默认使用当前目录
            error_message: None,
        }
    }

    /// 设置错误信息
    fn set_error(&mut self, error: impl ToString) {
        self.error_message = Some(error.to_string());
    }

    /// 加载文件内容
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
// 实现eframe::App，eframe egui库的框架
impl eframe::App for DirectoryApp {
    // 更新界面，每一帧都会执行一次
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 左侧面板：文件浏览器
        egui::SidePanel::left("File browser")
            .default_width(200.0)
            .show(ctx, |ui| {
                // 顶部导航栏
                ui.horizontal(|ui| {
                    // 返回上级目录按钮
                    if ui.button(RichText::new("⬆").size(14.0)).clicked()
                        && self.current_dir.parent().is_some()
                    {
                        self.current_dir.pop();
                    }

                    // 显示当前路径
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        let path_text = self.current_dir.to_string_lossy().to_string();
                        ui.label(RichText::new(path_text).size(11.0));
                    });
                });

                ui.separator();

                // 文件列表区域
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Ok(read_dir) = read_dir(&self.current_dir) {
                        // 收集并排序目录条目
                        let mut entries: Vec<_> = read_dir.flatten().collect();
                        entries.sort_by(|a, b| {
                            // 首先按照类型排序(目录在前)
                            let a_is_dir = a.metadata().map(|m| m.is_dir()).unwrap_or(false);
                            let b_is_dir = b.metadata().map(|m| m.is_dir()).unwrap_or(false);

                            // 如果类型不同，目录排在前面
                            if a_is_dir != b_is_dir {
                                return b_is_dir.cmp(&a_is_dir);
                            }

                            // 如果类型相同，按名称排序
                            a.file_name().cmp(&b.file_name())
                        });

                        // 显示排序后的条目
                        for entry in entries {
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

        // 中央面板：文件内容显示
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(error) = &self.error_message {
                // 显示错误信息
                ui.colored_label(Color32::RED, error);
            } else if !self.file_content.is_empty() {
                // 显示文件内容
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        TextEdit::multiline(&mut self.file_content)
                            .desired_width(f32::INFINITY)
                            .desired_rows(30)
                            .code_editor(),
                    );
                });
            } else {
                // 显示提示信息
                ui.centered_and_justified(|ui| {
                    ui.label("Select a file to view its contents");
                });
            }
        });
    }
}

/// 程序入口点
fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "File explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(DirectoryApp::new(cc)))),
    );
}
