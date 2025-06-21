use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

use smart_road::{
    intersection::Intersection,
    stats::Stats,
    vehicle::{Direction},
};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Road", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut intersection = Intersection::new();
    let mut stats = Stats::new();
    let mut last_update = Instant::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    intersection.spawn_vehicle(Direction::North)
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    intersection.spawn_vehicle(Direction::South)
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    intersection.spawn_vehicle(Direction::East)
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    intersection.spawn_vehicle(Direction::West)
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_update).as_secs_f64();
        last_update = now;
        intersection.update(dt, &mut stats);

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 200, 0));
        intersection.draw(&mut canvas)?;
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }

    println!("Vehicles passed: {}", stats.vehicles_passed);
    println!("Max velocity: {}", stats.max_velocity);
    println!("Min velocity: {}", stats.min_velocity);
    println!("Max time: {}", stats.max_time);
    println!("Min time: {}", stats.min_time);
    if stats.vehicles_passed > 0 {
        println!("Avg time: {}", stats.total_time / stats.vehicles_passed as f64);
    }

    Ok(())
}
