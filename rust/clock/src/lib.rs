const MINUTES_IN_HOUR: i32 = 60;
const MINUTES_IN_DAY: i32 = 24 * MINUTES_IN_HOUR;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::normalized(hours * MINUTES_IN_HOUR + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::normalized(self.0 + minutes)
    }

    fn normalized(minutes: i32) -> Self {
        Self({
            let wrapped_minutes = minutes % MINUTES_IN_DAY;
            if wrapped_minutes.is_negative() {
                MINUTES_IN_DAY + wrapped_minutes
            } else {
                wrapped_minutes
            }
        })
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.0 / MINUTES_IN_HOUR;
        let minutes = self.0 % MINUTES_IN_HOUR;
        write!(f, "{hours:<02}:{minutes:<02}")
    }
}
