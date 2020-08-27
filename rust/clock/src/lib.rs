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
        fn divrem(n: i32, d: i32) -> (i32, i32, i32) {
            (n / d, n % d, n.rem_euclid(d))
        }

        let (h_carry, m_neg, m) = divrem(minutes, 60);
        let (_, _, h) = divrem(hours + h_carry + (if m_neg < 0 { -1 } else { 0 }), 24);
        (h, m)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}