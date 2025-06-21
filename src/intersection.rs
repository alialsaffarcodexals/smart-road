use crate::vehicle::{Route, Vehicle};
use sdl2::render::Canvas;
use sdl2::video::Window;

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
        let (x, y) = match route {
            // Spawn from the bottom moving up
            Route::Straight => (310.0, 460.0),
            // Spawn from the right moving left
            Route::Left => (620.0, 220.0),
            // Spawn from the left moving right
            Route::Right => (0.0, 220.0),
        };
        let v = Vehicle::new(self.counter, x, y, 50.0, route);
        self.counter += 1;
        self.vehicles.push(v);
    }

    pub fn update(&mut self, dt: f64) {
        for v in &mut self.vehicles {
            v.update(dt);
        }
        self.vehicles.retain(|v| v.x >= 0.0 && v.x <= 640.0 && v.y >= 0.0 && v.y <= 480.0);
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        for v in &self.vehicles {
            canvas.fill_rect(v.sprite)?;
        }
        Ok(())
    }
}
