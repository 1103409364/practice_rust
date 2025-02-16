use std::fmt::Display;

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i64,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(&mut self, minutes: i64) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        Self::new(0, self.minutes + minutes)
    }
}
