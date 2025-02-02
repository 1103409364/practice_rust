// NothingApp 结构体定义了应用程序的基本状态
#[derive(Default)]
struct NothingApp {
    number: i32,  // 计数器值
    text: String, // 文本编辑区内容
    code: String, // 代码编辑区内容
}

impl NothingApp {
    // 创建并初始化一个新的 NothingApp 实例
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            number: 0,                                                         // 初始计数器值为0
            text: String::from("Put some text in here!"),                      // 默认文本
            code: String::from(r#"fn main() { println!("Hello, world!"); }"#), // 默认代码
        }
    }
}

impl eframe::App for NothingApp {
    // 更新UI界面的函数
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 增加计数器按钮
            if ui.button("Counter up").clicked() {
                self.number += 1
            }
            // 减少计数器按钮
            if ui.button("Counter down").clicked() {
                self.number -= 1
            }
            // 显示当前计数器值
            ui.label(format!("The counter is: {}", self.number));
            // 多行文本编辑区
            ui.text_edit_multiline(&mut self.text);
            // 代码编辑器区域
            ui.code_editor(&mut self.code);
        });
    }
}

// 主函数 - 程序入口点
fn main() {
    // 创建默认的原生窗口配置
    let native_options = eframe::NativeOptions::default();
    // 运行GUI应用程序
    let _ = eframe::run_native(
        "My egui App", // 窗口标题
        native_options,
        Box::new(|cc| Ok(Box::new(NothingApp::new(cc)))),
    );
}
