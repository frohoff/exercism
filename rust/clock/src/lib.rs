use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes: (60 * hours + minutes).rem_euclid(24 * 60) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h = (self.minutes / 60).rem_euclid(24);
        let m = self.minutes.rem_euclid(60);
        write!(f, "{:02}:{:02}", h, m)
    }
}