use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Clock {
            minutes: Self::get_minutes(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        Clock {
            minutes: Self::get_minutes(self.minutes + minutes),
        }
    }
    fn get_minutes(minutes: i32) -> i32 {
        match minutes {
            x if x < 0 => x % (24 * 60) + 24 * 60,
            x => x % (24 * 60),
        }
    }
}
