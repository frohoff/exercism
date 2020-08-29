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

macro_rules! planet {
    ($id:ident, $e:expr) => {
        pub struct $id;
        impl Planet for $id { fn factor() -> f64 { $e } }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);