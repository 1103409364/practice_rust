use ansi_term::Colour::Red;
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::{fs::read_to_string, io::stdout};

struct App {
    file_content: String,
    user_input: String,
}

impl App {
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(file_name)?;
        Ok(Self {
            file_content,
            user_input: String::new(),
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut app = App::new("typing.txt")?;
    // 记录开始时间
    let start_time = std::time::Instant::now();
    loop {
        println!("{}", app.file_content);
        for (letter1, letter2) in app.user_input.chars().zip(app.file_content.chars()) {
            if letter1 == letter2 {
                print!("{letter2}");
            } else {
                print!("{}", Red.paint(String::from(letter1)));
            }
        }
        println!("_");
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.user_input.pop();
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        app.user_input.push(c);
                    }
                    KeyCode::Enter => {
                        let total_words = app.file_content.split_whitespace().count();
                        let mut total_words_right = 0;
                        for (word1, word2) in app
                            .user_input
                            .split_whitespace()
                            .zip(app.file_content.split_whitespace())
                        {
                            // 去除标点符号 使用正则表达式
                            let word1 = word1.trim_matches(|c: char| !c.is_alphabetic());
                            let word2 = word2.trim_matches(|c: char| !c.is_alphabetic());
                            if word1 == word2 {
                                total_words_right += 1;
                            }
                        }
                        let elapsed_time = start_time.elapsed().as_secs_f64();
                        let words_per_minute = (total_words_right as f64 / elapsed_time) * 60.0;
                        println!("You got {total_words_right} out of {total_words}! You took {elapsed_time} seconds. Your typing speed is {words_per_minute} words per minute.");
                        return Ok(());
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All))?;
        }
    }
    Ok(())
}
