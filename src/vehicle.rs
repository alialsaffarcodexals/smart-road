use sdl2::rect::Rect;
use std::time::Instant;

/// Cardinal directions a vehicle can face while driving.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

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
    pub direction: Direction,
    pub route: Route,
    pub turned: bool,
    pub start_time: Instant,
    pub sprite: Rect, // simple rectangle representing the vehicle
}

impl Vehicle {
    pub fn new(
        id: usize,
        x: f64,
        y: f64,
        velocity: f64,
        direction: Direction,
        route: Route,
    ) -> Self {
        Vehicle {
            id,
            x,
            y,
            velocity,
            direction,
            route,
            turned: false,
            start_time: Instant::now(),
            sprite: Rect::new(x as i32, y as i32, 20, 40),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let dist = self.velocity * dt;
        match self.direction {
            Direction::North => self.y -= dist,
            Direction::South => self.y += dist,
            Direction::East => self.x += dist,
            Direction::West => self.x -= dist,
        }

        // Change direction at the intersection centre according to route.
        const CENTER_X: f64 = 320.0;
        const CENTER_Y: f64 = 240.0;
        if !self.turned
            && (self.x - CENTER_X).abs() < 10.0
            && (self.y - CENTER_Y).abs() < 10.0
        {
            match self.route {
                Route::Straight => {}
                Route::Left => self.direction = self.direction.turn_left(),
                Route::Right => self.direction = self.direction.turn_right(),
            }
            self.turned = true;
        }

        self.sprite.set_x(self.x as i32);
        self.sprite.set_y(self.y as i32);
    }
}
