use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (((((hours as f32) + (minutes as f32 / 60f32)) % 24f32) + 24f32) % 24f32) as u8,
            minutes: (((minutes % 60) + 60) % 60) as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = (self.minutes as i32) + minutes;
        Clock {
            hours: (((((self.hours as f32) + (new_minutes as f32 / 60f32)) % 24f32) + 24f32)
                % 24f32) as u8,
            minutes: (((new_minutes % 60) + 60) % 60) as u8,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
