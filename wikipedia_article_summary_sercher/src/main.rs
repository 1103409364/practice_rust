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
}
#[derive(Debug, Default)]
struct App {
    language: String,
    current_article: CurrentArticle,
    search_string: String,
    error: Option<String>,
    uri: Option<String>,
    method: Option<String>,
}
impl App {
    fn get_article(&mut self) -> Result<(), Box<dyn Error>> {
        match get(format!(
            "https://{}.{URL}/{}",
            self.language, self.search_string
        )) {
            Ok(res) => {
                let text = res.text()?;
                if let Ok(article) = serde_json::from_str::<CurrentArticle>(&text) {
                    self.current_article = article;
                }
            }
            Err(error) => self.error = Some(error.to_string()),
        }

        Ok(())
    }
}
impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Language: {} \nSearching for: {}\nTitle: {}\nDescription: {}\nExtract: {}\nType: {} \nDetail: {} \nUri: {} \nMethod: {} \nErrorMessage: {}",
            self.language,
            self.search_string,
            self.current_article.title.as_deref().unwrap_or(""),
            self.current_article.description.as_deref().unwrap_or(""),
            self.current_article.extract.as_deref().unwrap_or(""),
            self.current_article.r#type.as_deref().unwrap_or(""),
            self.current_article.detail.as_deref().unwrap_or(""),
            self.uri.as_deref().unwrap_or(""),
            self.method.as_deref().unwrap_or(""),
            self.error.as_deref().unwrap_or("")
        )
    }
}
const URL: &str = "wikipedia.org/api/rest_v1/page/summary";
const LANGUAGES: [&str; 2] = ["zh", "en"];

fn handle_language(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Please enter correct language!");
        println!("{app}");
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.language.pop();
                    }
                    KeyCode::Esc => app.language.clear(),
                    KeyCode::Enter => {
                        if LANGUAGES.iter().any(|&s| s == app.language) {
                            return Ok(());
                        }
                    }
                    KeyCode::Char(c) => {
                        app.language.push(c);
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All))?;
        }
    }
}

fn handle_search(app: &mut App) -> Result<(), Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();
    match handle_language(&mut app) {
        Ok(()) => handle_search(&mut app),
        Err(e) => Err(e),
    }
}
