use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::extract::State;
use axum::{extract::Path, routing::get, Router}; //  http::StatusCode, 导入必要的 axum 模块
use serde::{Deserialize, Serialize};
use time::Duration;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

const GAME_GUESS_KEY: &str = "/game/guess";

// 定义一个包含随机单词的常量数组
const RANDOM_WORDS: [&str; 6] = ["MB", "Windy", "Gomes", "Johnny", "Seoul", "Interesting"];

// 定义游戏状态结构体
#[derive(Debug, Deserialize, Serialize)]
struct GameApp {
    current_word: String,     // 当前要猜测的单词
    right_guesses: Vec<char>, // 正确猜测的字母
    wrong_guesses: Vec<char>, // 错误猜测的字母
}
// 自定义 Default 自动初始化和启动游戏
impl Default for GameApp {
    fn default() -> Self {
        let mut game = Self {
            current_word: String::new(),
            right_guesses: vec![],
            wrong_guesses: vec![],
        };
        game.restart();
        game
    }
}

// 定义猜测结果枚举
enum Guess {
    Right,          // 猜测正确
    Wrong,          // 猜测错误
    AlreadyGuessed, // 已经猜过
}

#[derive(Clone)]
struct AppState {
    game_locks: Arc<Mutex<HashMap<String, ()>>>, // 用户会话锁映射
}

// 从状态中获取游戏结果的异步函数 Path 路径解析器
async fn get_res_from_state(
    State(state): State<AppState>,
    Path(guess): Path<String>,
    session: Session,
) -> Result<String, String> {
    let session_id = session.id().expect("Session ID must exist").to_string();

    // 使用 session_id 获取或创建用户特定的锁
    let mut locks = state.game_locks.lock().await;
    locks.entry(session_id).or_default();

    // 现在这个会话的所有操作都是原子的
    let mut game_app = match session.get::<GameApp>(GAME_GUESS_KEY).await {
        Ok(Some(app)) => app,
        Ok(None) => GameApp::default(),
        Err(_) => GameApp::default(),
    };

    let res = game_app.take_guess(guess);

    if let Err(_) = session.insert(GAME_GUESS_KEY, game_app).await {
        return Ok("Error saving game state, but here's your result: ".to_string() + &res);
    }

    Ok(res)
}

impl GameApp {
    // 重置游戏状态
    fn restart(&mut self) {
        self.current_word = RANDOM_WORDS[fastrand::usize(..RANDOM_WORDS.len())].to_lowercase(); // 随机选择一个单词
        self.right_guesses.clear(); // 清空正确猜测的字母
        self.wrong_guesses.clear(); // 清空错误猜测的字母
    }
    // 检查猜测是否正确
    fn check_guess(&self, guess: char) -> Guess {
        if self.right_guesses.contains(&guess) || self.wrong_guesses.contains(&guess) {
            return Guess::AlreadyGuessed; // 如果已经猜过，返回 AlreadyGuessed
        }
        match self.current_word.contains(guess) {
            true => Guess::Right,  // 如果单词包含猜测的字母，返回 Right
            false => Guess::Wrong, // 如果单词不包含猜测的字母，返回 Wrong
        }
    }
    // 返回当前猜测的结果
    fn results_so_far(&self) -> String {
        let mut output = String::new();
        for c in self.current_word.chars() {
            if self.right_guesses.contains(&c) {
                output.push(c) // 如果字母被正确猜测，则显示字母
            } else {
                output.push('*') // 否则显示 *
            }
        }
        output
    }

    // 处理猜测
    fn take_guess(&mut self, guess: String) -> String {
        println!("{self:?}"); // 打印当前游戏状态

        let guess = guess.to_lowercase(); // 将猜测转换为小写
        let mut output = String::new();
        match guess {
            // 匹配守卫，字符长度为 1 时
            guess if guess.chars().count() == 1 => {
                let the_guess = guess.chars().next().unwrap(); // 获取猜测的字符 next() 方法是迭代器上的一个方法。它会尝试从迭代器中取出下一个元素
                match self.check_guess(the_guess) {
                    Guess::AlreadyGuessed => {
                        output.push_str(&format!("You already guessed {the_guess}!\n"));
                        // 如果已经猜过，则提示
                    }
                    Guess::Right => {
                        self.right_guesses.push(the_guess); // 将正确猜测的字母添加到 right_guesses
                        output.push_str(&format!("Yes, it contains a {the_guess}!\n"));
                        // 提示猜测正确
                    }
                    Guess::Wrong => {
                        self.wrong_guesses.push(the_guess); // 将错误猜测的字母添加到 wrong_guesses
                        output.push_str(&format!("Nope, it doesn't contain a {the_guess}!\n"));
                        // 提示猜测错误
                    }
                }
                output.push_str(&self.results_so_far()); // 添加当前猜测结果
            }
            // 字符长度不为 1 时
            guess => {
                if self.current_word == guess {
                    output.push_str(&format!(
                        "You guessed right, it's {}! Let's play again!",
                        self.current_word
                    )); // 如果猜测的单词正确，则提示并重置游戏
                } else {
                    output.push_str(&format!(
                        "Bzzt! It's not {guess}, it's {}.\nTime to move on to another word!",
                        self.current_word
                    )); // 如果猜测的单词错误，则提示并重置游戏
                }
                self.restart(); // 重置游戏
            }
        }
        output
    }
}

// 基础处理器，返回静态字符串
async fn root() -> &'static str {
    "Hello, World!" // 返回 Hello, World!
}

// 等效于闭包 get(|Path(guess): Path<String>| async move { format!("The guess is {guess}") }),
// async fn guess(Path(guess): Path<String>) -> String {
//     format!("The guess is {guess} StatusCode {}", StatusCode::OK)
// }
// 注释掉的 guess 函数，用于演示路径参数

// 处理 double 路径的异步函数
async fn double(Path(input): Path<String>) -> String {
    match input.parse::<i32>() {
        Ok(num) => format!("{} times 2 is {}!", num, num * 2), // 如果输入是数字，则返回其两倍
        Err(e) => format!("Uh oh, weird input: {e}"),          // 如果输入不是数字，则返回错误信息
    }
}

// 主函数
#[tokio::main]
async fn main() {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(1000 * 60)));

    let app_state = AppState {
        game_locks: Arc::new(Mutex::new(HashMap::new())),
    };

    // 构建应用路由
    let app = Router::new()
        // `GET /` 路由到 `root` 函数
        .route("/", get(root))
        .route("/game/{guess}", get(get_res_from_state)) // `GET /game/{guess}` 路由到 `get_res_from_state` 函数
        .route("/double/{number}", get(double)) // `GET /double/{number}` 路由到 `double` 函数
        .layer(session_layer)
        .with_state(app_state);

    // 使用 hyper 运行应用，监听 8080 端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
