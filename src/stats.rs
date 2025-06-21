use crate::vehicle::Vehicle;

#[derive(Default, Debug)]
pub struct Stats {
    pub vehicles_passed: usize,
    pub max_velocity: f64,
    pub min_velocity: f64,
    pub max_time: f64,
    pub min_time: f64,
    pub total_time: f64,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            vehicles_passed: 0,
            max_velocity: 0.0,
            min_velocity: f64::MAX,
            max_time: 0.0,
            min_time: f64::MAX,
            total_time: 0.0,
        }
    }

    pub fn register_vehicle(&mut self, v: &Vehicle, time: f64) {
        self.vehicles_passed += 1;
        if v.velocity > self.max_velocity {
            self.max_velocity = v.velocity;
        }
        if v.velocity < self.min_velocity {
            self.min_velocity = v.velocity;
        }
        if time > self.max_time {
            self.max_time = time;
        }
        if time < self.min_time {
            self.min_time = time;
        }
        self.total_time += time;
    }
}
