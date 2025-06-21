use sdl2::rect::Rect;

#[derive(Clone, Copy, Debug)]
pub enum Route {
    Right,
    Straight,
    Left,
}

#[derive(Clone, Debug)]
pub struct Vehicle {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub velocity: f64,
    pub route: Route,
    pub sprite: Rect, // simple rectangle representing the vehicle
}

impl Vehicle {
    pub fn new(id: usize, x: f64, y: f64, velocity: f64, route: Route) -> Self {
        Vehicle {
            id,
            x,
            y,
            velocity,
            route,
            sprite: Rect::new(x as i32, y as i32, 20, 40),
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Very simple movement: move up if route is Straight, etc.
        match self.route {
            Route::Right => {
                self.x += self.velocity * dt;
            }
            Route::Straight => {
                self.y -= self.velocity * dt;
            }
            Route::Left => {
                self.x -= self.velocity * dt;
            }
        }
        self.sprite.set_x(self.x as i32);
        self.sprite.set_y(self.y as i32);
    }
}
