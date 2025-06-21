use crate::stats::Stats;
use crate::vehicle::{Direction, Route, Vehicle};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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

    /// Spawn a vehicle coming from `direction`. The route is chosen randomly.
    pub fn spawn_vehicle(&mut self, direction: Direction) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let route = match rng.gen_range(0..3) {
            0 => Route::Straight,
            1 => Route::Left,
            _ => Route::Right,
        };

        const SPAWN_OFFSET: f64 = 40.0;
        let (x, y) = match direction {
            Direction::North => (320.0, -SPAWN_OFFSET),
            Direction::South => (320.0, 480.0 + SPAWN_OFFSET),
            Direction::East => (-SPAWN_OFFSET, 240.0),
            Direction::West => (640.0 + SPAWN_OFFSET, 240.0),
        };

        let v = Vehicle::new(self.counter, x, y, 60.0, direction, route);
        self.counter += 1;
        self.vehicles.push(v);
    }

    pub fn update(&mut self, dt: f64, stats: &mut Stats) {
        for v in &mut self.vehicles {
            v.update(dt);
        }
        let mut i = 0;
        while i < self.vehicles.len() {
            let v = &self.vehicles[i];
            if v.x < -50.0 || v.x > 690.0 || v.y < -50.0 || v.y > 530.0 {
                let v = self.vehicles.remove(i);
                let time = v.start_time.elapsed().as_secs_f64();
                stats.register_vehicle(&v, time);
            } else {
                i += 1;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // draw roads
        canvas.set_draw_color(Color::RGB(60, 60, 60));
        canvas.fill_rect(Rect::new(300, 0, 80, 480))?; // vertical road
        canvas.fill_rect(Rect::new(0, 200, 640, 80))?; // horizontal road

        // draw vehicles
        canvas.set_draw_color(Color::RGB(0, 200, 0));
        for v in &self.vehicles {
            canvas.fill_rect(v.sprite)?;
        }
        Ok(())
    }
}
