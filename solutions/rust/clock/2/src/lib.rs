use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = (hours * 60 + minutes) % 1440;
        if minutes > 0 {
            Self {minutes}
        } else {
            let minutes = minutes +1440;
            Self {minutes}
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = minutes % 1440;
        if self.minutes + minutes > 0 {
            Self {
                minutes: self.minutes + minutes,
            } 
        } else {
            Self {
                minutes: self.minutes + minutes + 1440,
            } 
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = &self.minutes / 60;
        let hours = hours % 24;
        let minutes = &self.minutes % 60;
        write!(f, "{hours:02}:{minutes:02}")
    }
}
