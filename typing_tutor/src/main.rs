use ansi_term::Colour::Red;
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::collections::HashMap;
use std::{fs::read_to_string, io::stdout};

struct App {
    file_content: String,
    user_input: String,
    // 特殊符号 map
    char_map: HashMap<char, HashMap<char, char>>,
}

impl App {
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(file_name)?;
        Ok(Self {
            file_content,
            user_input: String::new(),
            char_map: Self::get_char_map(),
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
    fn handle_char(&mut self, c: char) {
        // 处理特殊字符，例如重音符号
        if let Some(map) = self.char_map.get(&c) {
            let pre_char = self.user_input.pop();
            if let Some(pre_char) = pre_char {
                if let Some(new_char) = map.get(&pre_char) {
                    self.push_char(*new_char);
                } else {
                    self.push_char(pre_char);
                    self.push_char(c);
                }
            } else {
                self.push_char(c);
            }
        } else {
            self.push_char(c);
        }
    }
    // 获取特殊字符 map
    fn get_char_map() -> HashMap<char, HashMap<char, char>> {
        let mut map = HashMap::new();
        for (_i, c) in "'`^~\"".chars().enumerate() {
            let mut inner_map = HashMap::new();
            match c {
                '\'' => {
                    // for (letter1, letter2) in "aeiouAEIOU".chars().zip("áéíóúÁÉÍÓÚ".chars())
                    // {
                    //     inner_map.insert(letter1, letter2);
                    // }
                    "aeiouAEIOU".chars().zip("áéíóúÁÉÍÓÚ".chars()).for_each(
                        |(letter1, letter2)| {
                            inner_map.insert(letter1, letter2);
                        },
                    );
                }
                '`' => {
                    // 改用 forEach
                    "aeiouAEIOU".chars().zip("àèìòùÀÈÌÒÙ".chars()).for_each(
                        |(letter1, letter2)| {
                            inner_map.insert(letter1, letter2);
                        },
                    );
                }
                '^' => {
                    "aeiouAEIOU".chars().zip("âêîôûÂÊÎÔÛ".chars()).for_each(
                        |(letter1, letter2)| {
                            inner_map.insert(letter1, letter2);
                        },
                    );
                }
                '~' => {
                    "aeiouAEIOU".chars().zip("ãẽĩõũÃẼĨÕŨ".chars()).for_each(
                        |(letter1, letter2)| {
                            inner_map.insert(letter1, letter2);
                        },
                    );
                }
                '"' => {
                    "aeiouAEIOU".chars().zip("äëïöüÄËÏÖÜ".chars()).for_each(
                        |(letter1, letter2)| {
                            inner_map.insert(letter1, letter2);
                        },
                    );
                }
                _ => {}
            }
            map.insert(c, inner_map);
        }
        map
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
                        app.handle_char(c);
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
