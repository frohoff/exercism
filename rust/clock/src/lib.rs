use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Clock::normalize(hours, minutes);
        Clock { hours: h, minutes: m}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, m) = Clock::normalize(self.hours, self.minutes + minutes);        
        Clock::new(h, m)
    }

    fn normalize(hours: i32, minutes: i32) -> (i32, i32) {
        let (m_addl_hours, m_rem, m_mod) = (minutes / 60, minutes % 60, minutes.rem_euclid(60));       
        let h = (hours + m_addl_hours + (if m_rem < 0 { -1 } else { 0 })).rem_euclid(24);
        (h, m_mod)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}