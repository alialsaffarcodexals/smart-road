use crate::vehicle::{Route, Vehicle};

#[derive(Default)]
pub struct Intersection {
    pub vehicles: Vec<Vehicle>,
    pub counter: usize,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            vehicles: Vec::new(),
            counter: 0,
        }
    }

    pub fn spawn_vehicle(&mut self, route: Route) {
        let v = Vehicle::new(self.counter, 300.0, 300.0, 50.0, route);
        self.counter += 1;
        self.vehicles.push(v);
    }

    pub fn update(&mut self, dt: f64) {
        for v in &mut self.vehicles {
            v.update(dt);
        }
        self.vehicles.retain(|v| v.x >= 0.0 && v.x <= 640.0 && v.y >= 0.0 && v.y <= 480.0);
    }
}
