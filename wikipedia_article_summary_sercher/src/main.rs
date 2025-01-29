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
    // &mut self 是 self: &mut Self 的简写
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
    // cannot borrow `*self` as mutable more than once at a time 这个方法不能加 &mut self
    fn handle_input(target: &mut String) -> Result<bool, Box<dyn Error>> {
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        target.pop();
                    }
                    KeyCode::Esc => target.clear(),
                    KeyCode::Enter => return Ok(true),
                    KeyCode::Char(c) => {
                        target.push(c);
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All))?;
        }
        Ok(false)
    }

    fn set_language(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            println!("Please enter correct language!");
            println!("{self}");
            if App::handle_input(&mut self.language)? {
                if LANGUAGES.iter().any(|&s| s == self.language) {
                    return Ok(());
                }
            }
        }
    }

    fn search(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            println!("{self}");
            if App::handle_input(&mut self.search_string)? {
                self.get_article()?;
            }
        }
    }
}
impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Searching for: {}", self.search_string)?;

        // Article fields
        let article_fields = [
            ("Title", &self.current_article.title),
            ("Description", &self.current_article.description),
            ("Extract", &self.current_article.extract),
            ("Type", &self.current_article.r#type),
            ("Detail", &self.current_article.detail),
        ];

        // App fields
        let app_fields = [
            ("Uri", &self.uri),
            ("Method", &self.method),
            ("ErrorMessage", &self.error),
        ];

        // Print article fields
        for (name, value) in article_fields.iter() {
            writeln!(f, "{}: {}", name, value.as_deref().unwrap_or(""))?;
        }

        // Print app fields
        for (name, value) in app_fields.iter() {
            writeln!(f, "{}: {}", name, value.as_deref().unwrap_or(""))?;
        }

        Ok(())
    }
}
const URL: &str = "wikipedia.org/api/rest_v1/page/summary";
const LANGUAGES: [&str; 2] = ["zh", "en"];

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();
    match app.set_language() {
        Ok(()) => app.search(), // 取引用传给方法
        Err(e) => Err(e),
    }
}
