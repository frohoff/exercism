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

macro_rules! planets {
    ($($id:ident: $e:literal),+) => { $(
        pub struct $id;
        impl Planet for $id { fn factor() -> f64 { $e } }
    )* }
}

planets!(Mercury: 0.2408467, Venus: 0.61519726, Earth: 1.0, Mars: 1.8808158, 
        Jupiter: 11.862615, Saturn: 29.447498, Uranus: 84.016846, Neptune: 164.79132);