// 引入必要的库
use chrono::{offset::Utc, FixedOffset, TimeZone};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph},
    Frame, Terminal,
};
use reqwest::blocking::get; // 使用同步方法，阻塞主线程
use serde::{Deserialize, Serialize};
use std::{
    io::stdout,
    thread::sleep,
    time::{Duration, Instant},
};

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
            display: String::from("00:00:00"), // 初始化显示时间为 0:00:00
        }
    }
    // 获取当前时间
    fn get_time(&self) -> String {
        use StopwatchState::*;
        match self.state {
            NotStarted => String::from("00:00:00"), // 如果未启动，返回 0:00:00
            Running => {
                // 如果正在运行，计算经过的时间
                let mut elapsed = self.now.elapsed().as_millis();
                let minutes = elapsed / 60000;
                elapsed -= minutes * 60000;
                let seconds = elapsed / 1000;
                elapsed -= seconds * 1000;
                let split_seconds = elapsed / 10;
                // 指定最小宽度 2，> 右对齐，并且使用 0 在左侧填充
                format!("{minutes:0>2}:{seconds:0>2}:{split_seconds:0>2}") // 格式化时间为 分:秒:毫秒
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
    // east_opt 参数毫秒 east 已经废弃
    let beijing_offset = FixedOffset::east_opt(8 * 3600).expect("FixedOffset::east out of bounds");
    let beijing_time = beijing_offset
        .from_utc_datetime(&Utc::now().naive_utc())
        .format("%Y/%m/%d %H:%M:%S")
        .to_string();

    let london_time = Utc::now().format("%Y/%m/%d %H:%M:%S").to_string();
    format!("{london_time}\n{beijing_time} Bei Jing")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub current_units: CurrentUnits,
    pub current: Current,
    pub hourly_units: HourlyUnits,
    pub hourly: Hourly,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUnits {
    pub time: String,
    pub interval: String,
    pub temperature_2m: String,
    pub wind_speed_10m: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub time: String,
    pub interval: i32,
    pub temperature_2m: f64,
    pub wind_speed_10m: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub relative_humidity_2m: String,
    pub wind_speed_10m: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub relative_humidity_2m: Vec<i32>,
    pub wind_speed_10m: Vec<f64>,
}

//
fn get_weather() -> Result<WeatherData, anyhow::Error> {
    match get("https://api.open-meteo.com/v1/forecast?latitude=28.23&longitude=112.94&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m") {
        Ok(res) => {
            let text = res.text()?;
            let data = serde_json::from_str::<WeatherData>(&text)?;
            // println!("{:?}", data);
            return Ok(data);
        }
        Err(e) => return Err(e.into()),
    }
}
fn ui(
    f: &mut Frame,
    stopwatch: &Stopwatch,
    weather_data: &WeatherData,
    // Box<dyn std::error::Error> anyhow::Error
) -> Result<(), anyhow::Error> {
    // First split into 2 rows (60% and 40%)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(f.area());

    // Split the first row into 2 equal columns
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    // top_chunks[0] - top left
    // top_chunks[1] - top right
    // main_chunks[1] - bottom row
    let stopwatch_area = top_chunks[0]; // 左侧区域用于显示秒表
    let utc_time_area = top_chunks[1]; // 右侧区域用于显示UTC时间
    let wether_area = main_chunks[1]; // 天气

    // Example usage with blocks:
    let stopwatch_block = block_with("Stopwatch"); // 创建秒表块
    let utc_time_block = block_with("Time in London"); // 创建UTC时间块
    let wether_block = block_with("Wether");

    let stopwatch_text = Paragraph::new(stopwatch.get_time()).block(stopwatch_block); // 创建秒表文本
    let utc_text = Paragraph::new(utc_pretty()).block(utc_time_block); // 创建UTC时间文本

    // let weather_text = serde_json::to_string(&weather_data)?;
    // let wether_txt = Paragraph::new(weather_text).block(wether_block);

    // Create data points for the chart
    let data_points: Vec<(f64, f64)> = weather_data.hourly.temperature_2m[0..24]
        .iter()
        .enumerate()
        .map(|(i, temp)| (i as f64, *temp))
        .collect();

    // Create the dataset for the temperature
    let datasets = vec![Dataset::default()
        .name("Temperature (°C)")
        .marker(ratatui::symbols::Marker::Dot)
        .data(&data_points)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title("Time (hours)".red())
        .style(Style::default().white())
        .bounds([0.0, 23.0])
        .labels((0..=23).map(|i| format!("{i:0>2}:00")).collect::<Vec<_>>());

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("Temperature".red())
        .style(Style::default().white())
        .bounds([0.0, 50.0])
        .labels((0..=4).map(|i| (i * 10).to_string()).collect::<Vec<_>>());

    // Create the chart widget
    let wether_chart = Chart::new(datasets)
        .block(wether_block)
        .x_axis(x_axis)
        .y_axis(y_axis);

    // Render all widgets
    f.render_widget(stopwatch_text, stopwatch_area);
    f.render_widget(utc_text, utc_time_area);
    f.render_widget(wether_chart, wether_area);
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let stdout = stdout(); // 获取标准输出
    let backend = CrosstermBackend::new(stdout); // 创建 Crossterm 后端
    let mut terminal = Terminal::new(backend)?; // 创建终端
    let mut stopwatch = Stopwatch::new(); // 创建秒表实例
    let weather_data = get_weather()?;

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
            match ui(f, &stopwatch, &weather_data) {
                Ok(d) => d,
                Err(e) => {
                    println!("{e:?}");
                }
            };
        })?;
        sleep(Duration::from_millis(100)); // 休眠 100ms
        terminal.clear()?; // 清空终端
    }
}
