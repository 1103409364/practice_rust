use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::{error::Error, io::stdout};

#[derive(Debug, Serialize, Deserialize, Default)]
struct CurrentArticle {
    title: Option<String>,
    description: Option<String>,
    extract: Option<String>,
    r#type: Option<String>, // type 是关键字，用于定义类型别名。使用 r# 转义关键字
    detail: Option<String>,
    uri: Option<String>,
    method: Option<String>,
}
#[derive(Debug, Default)]
struct App {
    current_article: CurrentArticle,
    search_string: String,
}
impl App {
    fn get_article(&mut self) -> Result<(), Box<dyn Error>> {
        let text = get(format!("{URL}/{}", self.search_string))?.text()?;
        if let Ok(article) = serde_json::from_str::<CurrentArticle>(&text) {
            self.current_article = article;
        }
        Ok(())
    }
}
impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Searching for: {}\nTitle: {}\nDescription: {}\nextract: {}\ntype: {} \ndetail:{} \nuri:{} \nmethod: {}",
            self.search_string,
            self.current_article.title.as_deref().unwrap_or(""),
            self.current_article.description.as_deref().unwrap_or(""),
            self.current_article.extract.as_deref().unwrap_or(""),
            self.current_article.r#type.as_deref().unwrap_or(""),
            self.current_article.detail.as_deref().unwrap_or(""),
            self.current_article.uri.as_deref().unwrap_or(""),
            self.current_article.method.as_deref().unwrap_or("")
        )
    }
}
const URL: &str = "https://zh.wikipedia.org/api/rest_v1/page/summary";
fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();
    loop {
        println!("{app}");
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.search_string.pop();
                    }
                    KeyCode::Esc => app.search_string.clear(),
                    KeyCode::Enter => app.get_article()?,
                    KeyCode::Char(c) => {
                        app.search_string.push(c);
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All))?;
        }
    }
}
