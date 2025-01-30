// 引入必要的库
use chrono::offset::Utc;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io::stdout, thread::sleep, time::Duration, time::Instant};

// 定义秒表结构体
struct Stopwatch {
    now: Instant,          // 记录开始时间
    state: StopwatchState, // 记录秒表状态
    display: String,       // 记录显示的时间
}

// 定义秒表状态枚举
enum StopwatchState {
    NotStarted, // 秒表未启动
    Running,    // 秒表正在运行
    Done,       // 秒表已停止
}

impl Stopwatch {
    // 创建一个新的秒表实例
    fn new() -> Self {
        Self {
            now: Instant::now(),               // 初始化开始时间为当前时间
            state: StopwatchState::NotStarted, // 初始化状态为未启动
            display: String::from("0:00:00"),  // 初始化显示时间为 0:00:00
        }
    }
    // 获取当前时间
    fn get_time(&self) -> String {
        use StopwatchState::*;
        match self.state {
            NotStarted => String::from("0:00:00"), // 如果未启动，返回 0:00:00
            Running => {
                // 如果正在运行，计算经过的时间
                let mut elapsed = self.now.elapsed().as_millis();
                let minutes = elapsed / 60000;
                elapsed -= minutes * 60000;
                let seconds = elapsed / 1000;
                elapsed -= seconds * 1000;
                let split_seconds = elapsed / 10;
                format!("{minutes}:{seconds}:{split_seconds}") // 格式化时间为 分:秒:毫秒
            }
            Done => self.display.clone(), // 如果已停止，返回停止时的时间
        }
    }
    // 切换秒表状态
    fn next_state(&mut self) {
        use StopwatchState::*;
        match self.state {
            NotStarted => {
                // 如果未启动，设置为运行状态并记录开始时间
                self.now = Instant::now();
                self.state = Running;
            }
            Running => {
                // 如果正在运行，设置为停止状态并记录当前时间
                self.display = self.get_time();
                self.state = Done;
            }
            Done => self.state = NotStarted, // 如果已停止，设置为未启动状态
        }
    }
}
// 创建一个带有边框的块
fn block_with(input: &str) -> Block {
    Block::default().title(input).borders(Borders::ALL)
}
// 获取当前UTC时间并格式化为字符串
fn utc_pretty() -> String {
    Utc::now().format("%Y/%m/%d %H:%M:%S").to_string()
}

fn main() -> Result<(), anyhow::Error> {
    let stdout = stdout(); // 获取标准输出
    let backend = CrosstermBackend::new(stdout); // 创建 Crossterm 后端
    let mut terminal = Terminal::new(backend)?; // 创建终端
    let mut stopwatch = Stopwatch::new(); // 创建秒表实例
    loop {
        // 循环处理事件和绘制UI
        // poll 函数在这里用于【非阻塞】地检查是否有键盘事件发生。crossterm 库的 read 函数是阻塞的，如果没有事件发生，它会一直等待。为了避免程序在等待事件时卡住，poll 函数先检查是否有事件，如果有，read 函数才会读取事件。这样可以保证程序在没有事件时也能继续执行其他操作，例如更新UI。
        if poll(Duration::from_millis(0))? {
            // poll 函数检查是否有事件发生，参数为超时时间，(0) 表示超时时间为 0 毫秒，即立即返回。
            // 检查是否有事件发生
            if let Event::Key(key_event) = read()? {
                // 读取事件
                if let (KeyCode::Enter, KeyEventKind::Press) = (key_event.code, key_event.kind) {
                    // 如果是回车键按下，切换秒表状态
                    stopwatch.next_state();
                }
            }
        }
        terminal.draw(|f| {
            // 绘制UI
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(f.area()); // 将终端区域分割为左右两部分
            let stopwatch_area = layout[0]; // 左侧区域用于显示秒表
            let utc_time_area = layout[1]; // 右侧区域用于显示UTC时间
            let stopwatch_block = block_with("Stopwatch"); // 创建秒表块
            let utc_time_block = block_with("Time in London"); // 创建UTC时间块
            let stopwatch_text = Paragraph::new(stopwatch.get_time()).block(stopwatch_block); // 创建秒表文本
            let utc_text = Paragraph::new(utc_pretty()).block(utc_time_block); // 创建UTC时间文本
            f.render_widget(stopwatch_text, stopwatch_area); // 渲染秒表文本
            f.render_widget(utc_text, utc_time_area); // 渲染UTC时间文本
        })?;
        sleep(Duration::from_millis(100)); // 休眠 20ms
        terminal.clear()?; // 清空终端
    }
}
