use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from(hours * 60 + minutes)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::from(self.hours * 60 + self.minutes + minutes)
    }

    fn from(minutes: i32) -> Self {
        let mut h = minutes / 60 % 24;
        let mut m = minutes % 60;
        if m < 0 {
            h -= 1;
            m += 60;
        }
        if h < 0 {
            h += 24;
        }
        Clock { hours: h, minutes: m}
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> String {
        clock.to_string()
    }
}
