#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        const EARTH_ORBITAL_PERIOD: f64 = 31_557_600.0;
        Self(s as f64 / EARTH_ORBITAL_PERIOD)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

#[macro_export]
macro_rules! planet {
    ($planet: ident, $ratio: expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.0 / $ratio
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
