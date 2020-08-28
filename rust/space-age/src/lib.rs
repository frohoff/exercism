const SEC_PER_YEAR: f64 = 31_557_600f64;

#[derive(Debug)]
pub struct Duration {
    s: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

pub trait Planet {
    fn factor() -> f64;
    fn years_during(d: &Duration) -> f64 {
        (d.s as f64) / SEC_PER_YEAR / Self::factor()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury { fn factor() -> f64 { 0.2408467 } }
impl Planet for Venus { fn factor() -> f64 { 0.61519726 } }
impl Planet for Earth { fn factor() -> f64 { 1.0 } }
impl Planet for Mars { fn factor() -> f64 { 1.8808158 } }
impl Planet for Jupiter { fn factor() -> f64 { 11.862615 } }
impl Planet for Saturn { fn factor() -> f64 { 29.447498 } }
impl Planet for Uranus { fn factor() -> f64 { 84.016846 } }
impl Planet for Neptune { fn factor() -> f64 { 164.79132 } }