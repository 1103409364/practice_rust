use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use egui::{Color32, RichText, TextEdit};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{read, read_dir, read_to_string},
    path::PathBuf,
};

/// 目录浏览应用的主要结构
#[derive(Default)]
struct DirectoryApp {
    file_content: String,                  // 当前打开文件的内容
    current_dir: PathBuf,                  // 当前浏览的目录路径
    error_message: Option<String>,         // 错误信息，如果有的话
    current_file: Option<PathBuf>,         // 当前打开的文件路径
    is_modified: bool,                     // 新增：标记文件是否被修改
    show_save_dialog: bool,                // 新增：是否显示保存对话框
    pending_action: Option<PendingAction>, // 新增：待处理的动作
}

// 新增：定义待处理的动作
enum PendingAction {
    CloseFile,
    OpenFile(PathBuf),
}

impl DirectoryApp {
    /// 创建新的应用实例
    ///
    /// # 参数
    /// * `cc` - eframe创建上下文，用于初始化应用程序
    ///
    /// # 返回值
    /// 返回初始化后的DirectoryApp实例
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 设置字体支持中文字符
        let fonts = FontDefinitions::default();

        // 1. 加载外部字体文件
        // // Install my own font (maybe supporting non-latin characters):
        // fonts.font_data.insert(
        //     "SmileySans-Oblique".to_owned(),
        //     std::sync::Arc::new(
        //         // .ttf and .otf supported
        //         FontData::from_static(include_bytes!(concat!(
        //             env!("CARGO_MANIFEST_DIR"),
        //             "/assets/fonts/SmileySans-Oblique.otf"
        //         ))),
        //     ),
        // );

        // // Put my font first (highest priority):
        // // fonts
        // //     .families
        // //     .get_mut(&FontFamily::Proportional)
        // //     .unwrap()
        // //     .insert(0, "SmileySans-Oblique".to_owned());

        // // Put my font as last fallback for monospace:
        // fonts
        //     .families
        //     .get_mut(&FontFamily::Monospace)
        //     .unwrap()
        //     .push("SmileySans-Oblique".to_owned());

        // // 2. 加载系统字体
        // let font = std::fs::read("c:/Windows/Fonts/msyh.ttc").unwrap();
        // const FONT_SYSTEM_SANS_SERIF: &'static str = "Microsoft YaHei";

        // fonts.font_data.insert(
        //     FONT_SYSTEM_SANS_SERIF.to_owned(),
        //     FontData::from_owned(font).into(),
        // );

        // if let Some(vec) = fonts.families.get_mut(&FontFamily::Proportional) {
        //     vec.push(FONT_SYSTEM_SANS_SERIF.to_owned());
        // }

        // if let Some(vec) = fonts.families.get_mut(&FontFamily::Monospace) {
        //     vec.push(FONT_SYSTEM_SANS_SERIF.to_owned());
        // }

        // 初始化应用实例
        let mut app = Self {
            file_content: String::new(),
            current_dir: current_dir().unwrap_or_else(|_| PathBuf::from(".")), // 默认使用当前目录
            error_message: None,
            current_file: None,
            is_modified: false,      // 新增
            show_save_dialog: false, // 新增
            pending_action: None,    // 新增
        };

        // 3. 使用 load_system_fonts 方法加载系统字体 https://github.com/emilk/egui/discussions/1344
        cc.egui_ctx.set_fonts(app.load_system_fonts(fonts));
        app
    }

    /// 尝试从系统中加载指定字体族名称的字体
    ///
    /// # 参数
    /// * `family_names` - 字体族名称列表，按优先级排序
    ///
    /// # 返回值
    /// 返回Option<Vec<u8>>，成功时包含字体数据，失败时返回None
    fn load_font_family(&mut self, family_names: &[&str]) -> Option<Vec<u8>> {
        let system_source = SystemSource::new();

        for &name in family_names {
            match system_source
                .select_best_match(&[FamilyName::Title(name.to_string())], &Properties::new())
            {
                Ok(h) => match &h {
                    Handle::Memory { bytes, .. } => {
                        println!("Loaded {name} from memory.");
                        return Some(bytes.to_vec());
                    }
                    Handle::Path { path, .. } => {
                        println!("Loaded {name} from path: {:?}", path);
                        if let Ok(data) = read(path) {
                            return Some(data);
                        }
                    }
                },
                Err(e) => {
                    println!("Could not load {}: {:?}", name, e);
                    self.set_error(e);
                    return None;
                }
            }
        }

        None
    }

    /// 加载系统字体并配置字体定义
    ///
    /// # 参数
    /// * `fonts` - 初始字体定义
    ///
    /// # 返回值
    /// 返回更新后的FontDefinitions，包含所有加载的系统字体
    fn load_system_fonts(&mut self, mut fonts: FontDefinitions) -> FontDefinitions {
        let fontdb = HashMap::from([(
            "simplified_chinese",
            vec![
                "Microsoft YaHei",
                "SimSun",
                "PingFang SC", // 将更常用的字体移到前面
                "Source Han Sans CN",
                "Noto Sans CJK SC",
                // 移除重复的 "Noto Sans SC"
                // 移除不太常用的字体以提高加载效率
            ],
        )]);
        // fontdb.insert("korean", vec!["Source Han Sans KR"]);
        for (region, font_names) in fontdb {
            if let Some(font_data) = self.load_font_family(&font_names) {
                // info!("Inserting font {region}");
                fonts
                    .font_data
                    .insert(region.to_owned(), FontData::from_owned(font_data).into());

                // 简化重复代码
                for family in [FontFamily::Proportional, FontFamily::Monospace] {
                    if let Some(vec) = fonts.families.get_mut(&family) {
                        vec.push(region.to_owned());
                    }
                }
            }
        }

        fonts
    }

    /// 设置应用程序的错误信息
    ///
    /// # 参数
    /// * `error` - 实现了ToString trait的错误信息
    fn set_error(&mut self, error: impl ToString) {
        self.error_message = Some(error.to_string());
    }

    /// 加载并读取文件内容
    ///
    /// # 参数
    /// * `file_path` - 要加载的文件路径
    fn load_file(&mut self, file_path: PathBuf) {
        if self.is_modified {
            self.show_save_dialog = true;
            self.pending_action = Some(PendingAction::OpenFile(file_path));
            return;
        }
        self.load_file_internal(file_path);
    }

    /// 内部加载文件的方法
    fn load_file_internal(&mut self, file_path: PathBuf) {
        match read_to_string(&file_path) {
            Ok(content) => {
                self.file_content = content;
                self.current_file = Some(file_path); // 保存当前文件路径
                self.error_message = None;
                self.is_modified = false;
            }
            Err(e) => self.set_error(e),
        }
    }

    /// 渲染顶部导航栏
    fn render_navigation_bar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            // ui.set_min_height(24.0); // 设置最小高度

            if ui.button(RichText::new("⬆").size(14.0)).clicked()
                && self.current_dir.parent().is_some()
            {
                self.current_dir.pop();
            }

            egui::ScrollArea::horizontal().show(ui, |ui| {
                let path_text = self.current_dir.to_string_lossy().to_string();
                ui.label(RichText::new(path_text).size(12.0));
            });
        });
    }
    /// 获取文件/目录条目的颜色
    fn get_entry_style(&self, is_dir: bool, dark_mode: bool) -> Color32 {
        match (is_dir, dark_mode) {
            (true, true) => Color32::from_rgb(110, 166, 255),
            (true, false) => Color32::from_rgb(30, 100, 200),
            (false, true) => Color32::from_rgb(255, 210, 120),
            (false, false) => Color32::from_rgb(180, 140, 0),
        }
    }
    /// 渲染文件/目录条目
    fn render_directory_entry(&mut self, ui: &mut egui::Ui, name: String, is_dir: bool) {
        let icon = if is_dir { "📁 " } else { "📄 " };
        let color = self.get_entry_style(is_dir, ui.visuals().dark_mode);

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

    /// 渲染文件列表
    fn render_file_list(&mut self, ui: &mut egui::Ui) {
        if let Ok(read_dir) = read_dir(&self.current_dir) {
            let mut entries: Vec<_> = read_dir.flatten().collect();
            entries.sort_by(|a, b| {
                let a_is_dir = a.metadata().map(|m| m.is_dir()).unwrap_or(false);
                let b_is_dir = b.metadata().map(|m| m.is_dir()).unwrap_or(false);

                if a_is_dir != b_is_dir {
                    return b_is_dir.cmp(&a_is_dir);
                }
                a.file_name().cmp(&b.file_name())
            });

            for entry in entries {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(name) = entry.file_name().into_string() {
                        self.render_directory_entry(ui, name, metadata.is_dir());
                    }
                }
            }
        }
    }
    /// 渲染区顶栏
    fn render_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // 显示文件路径作为标题，显示修改状态
            if let Some(file_path) = &self.current_file {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    let title = if self.is_modified {
                        format!("*{}", file_path.to_string_lossy())
                    } else {
                        file_path.to_string_lossy().to_string()
                    };
                    ui.label(RichText::new(title).size(12.0));
                });
            }
            // 将关闭按钮、保存按钮放在最右边
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(RichText::new("❌").size(11.0)).clicked() {
                    if self.is_modified {
                        self.show_save_dialog = true;
                        self.pending_action = Some(PendingAction::CloseFile);
                    } else {
                        self.close_file();
                    }
                }
                if self.is_modified {
                    if ui.button(RichText::new("💾").size(11.0)).clicked() {
                        self.save_file();
                    }
                }
            });
        });
    }
    /// 渲染中央内容面板
    fn render_central_panel(&mut self, ui: &mut egui::Ui) {
        // 渲染保存对话框
        if self.show_save_dialog {
            egui::Window::new("Save Changes")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Do you want to save the changes?");
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            self.save_file();
                            self.show_save_dialog = false;
                            self.handle_pending_action();
                        }
                        if ui.button("Don't Save").clicked() {
                            self.is_modified = false;
                            self.show_save_dialog = false;
                            self.handle_pending_action();
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_save_dialog = false;
                            self.pending_action = None;
                        }
                    });
                });
        }

        if let Some(error) = &self.error_message {
            ui.colored_label(Color32::RED, error);
        } else if !self.file_content.is_empty() {
            self.render_top_bar(ui);

            ui.separator(); // 分割线

            // 文件内容显示
            egui::ScrollArea::vertical().show(ui, |ui| {
                let response = ui.add(
                    TextEdit::multiline(&mut self.file_content)
                        .desired_width(f32::INFINITY)
                        .desired_rows(30)
                        .code_editor(),
                );

                if response.changed() {
                    self.is_modified = true;
                }
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Select a file to view its contents");
            });
        }
    }

    // 新增：保存文件的方法
    fn save_file(&mut self) {
        if let Some(path) = &self.current_file {
            match std::fs::write(path, &self.file_content) {
                Ok(_) => {
                    self.is_modified = false;
                    self.error_message = None;
                }
                Err(e) => self.set_error(e),
            }
        }
    }

    // 新增：处理待处理的动作
    fn handle_pending_action(&mut self) {
        if let Some(action) = self.pending_action.take() {
            match action {
                PendingAction::CloseFile => self.close_file(),
                PendingAction::OpenFile(path) => self.load_file_internal(path),
            }
        }
    }

    // 新增：关闭文件的方法
    fn close_file(&mut self) {
        self.file_content.clear();
        self.current_file = None;
        self.is_modified = false;
    }
}

// 实现eframe::App，eframe egui库的框架
impl eframe::App for DirectoryApp {
    /// 更新UI界面，这是egui的主要渲染循环
    ///
    /// # 参数
    /// * `ctx` - egui上下文，用于绘制UI元素
    /// * `_frame` - eframe框架实例，用于控制窗口
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("File browser")
            .default_width(200.0)
            .show(ctx, |ui| {
                self.render_navigation_bar(ui);
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_file_list(ui);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_central_panel(ui);
        });
    }
}

/// 程序入口点
///
/// 初始化并运行文件浏览器应用程序
fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "File explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(DirectoryApp::new(cc)))),
    );
}
