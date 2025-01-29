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
    // push 特殊字符
    fn push_char(&mut self, c: char) {
        let char_len = self.file_content.chars().count();

        // 限制输入长度
        if self.user_input.chars().count() < char_len {
            self.user_input.push(c);
        };
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
                        // 处理特殊字符，例如重音符号
                        if ['\'', '`', '^', '~', '"'].contains(&c) {
                            let pre_char = app.user_input.pop();
                            match c {
                                '\'' => {
                                    if let Some(pre_char) = pre_char {
                                        match pre_char {
                                            'a' => app.push_char('á'),
                                            'e' => app.push_char('é'),
                                            'i' => app.push_char('í'),
                                            'o' => app.push_char('ó'),
                                            'u' => app.push_char('ú'),
                                            'A' => app.push_char('Á'),
                                            'E' => app.push_char('É'),
                                            'I' => app.push_char('Í'),
                                            'O' => app.push_char('Ó'),
                                            'U' => app.push_char('Ú'),
                                            _ => {
                                                app.push_char(pre_char);
                                                app.push_char(c);
                                            }
                                        }
                                    } else {
                                        app.push_char(c);
                                    }
                                }
                                '`' => {
                                    if let Some(pre_char) = pre_char {
                                        match pre_char {
                                            'a' => app.push_char('à'),
                                            'e' => app.push_char('è'),
                                            'i' => app.push_char('ì'),
                                            'o' => app.push_char('ò'),
                                            'u' => app.push_char('ù'),
                                            'A' => app.push_char('À'),
                                            'E' => app.push_char('È'),
                                            'I' => app.push_char('Ì'),
                                            'O' => app.push_char('Ò'),
                                            'U' => app.push_char('Ù'),
                                            _ => {
                                                app.push_char(pre_char);
                                                app.push_char(c);
                                            }
                                        }
                                    } else {
                                        app.push_char(c);
                                    }
                                }
                                '^' => {
                                    if let Some(pre_char) = pre_char {
                                        match pre_char {
                                            'a' => app.push_char('â'),
                                            'e' => app.push_char('ê'),
                                            'i' => app.push_char('î'),
                                            'o' => app.push_char('ô'),
                                            'u' => app.push_char('û'),
                                            'A' => app.push_char('Â'),
                                            'E' => app.push_char('Ê'),
                                            'I' => app.push_char('Î'),
                                            'O' => app.push_char('Ô'),
                                            'U' => app.push_char('Û'),
                                            _ => {
                                                app.push_char(pre_char);
                                                app.push_char(c);
                                            }
                                        }
                                    }
                                }
                                '~' => {
                                    if let Some(pre_char) = pre_char {
                                        match pre_char {
                                            'a' => app.push_char('ã'),
                                            'n' => app.push_char('ñ'),
                                            'o' => app.push_char('õ'),
                                            'A' => app.push_char('Ã'),
                                            'N' => app.push_char('Ñ'),
                                            'O' => app.push_char('Õ'),
                                            _ => {
                                                app.push_char(pre_char);
                                                app.push_char(c);
                                            }
                                        }
                                    }
                                }
                                '"' => {
                                    if let Some(pre_char) = pre_char {
                                        match pre_char {
                                            'a' => app.push_char('ä'),
                                            'e' => app.push_char('ë'),
                                            'i' => app.push_char('ï'),
                                            'o' => app.push_char('ö'),
                                            'u' => app.push_char('ü'),
                                            'y' => app.push_char('ÿ'),
                                            'A' => app.push_char('Ä'),
                                            'E' => app.push_char('Ë'),
                                            'I' => app.push_char('Ï'),
                                            'O' => app.push_char('Ö'),
                                            'U' => app.push_char('Ü'),
                                            'Y' => app.push_char('Ÿ'),
                                            _ => {
                                                app.push_char(pre_char);
                                                app.push_char(c);
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    app.push_char(c);
                                }
                            }
                        } else {
                            app.push_char(c);
                        }
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
