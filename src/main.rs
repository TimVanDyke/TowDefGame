extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

// use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    const TITLE: &str = "Protect Jo";

    let window = video_subsystem
        .window(TITLE, 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // fps Calc and Game Clock
    const FPS: u8 = 60;
    const NANOS: f32 = 1_000_000_000.0 / FPS as f32;
    let mut time_now: SystemTime;
    let mut timer = SystemTime::now();
    let mut last_time = SystemTime::now();
    let mut delta: f64 = 0.0;
    let mut clock: u8 = 0;
    let mut fps_counter = 0;
    let mut update_counter = 0;

    'running: loop {
        // fps and update timer goes here
        time_now = SystemTime::now();
        delta += time_now.duration_since(last_time).unwrap().as_nanos() as f64 / NANOS as f64;
        last_time = time_now;
        while delta > 1.0 {
            //manually updating and rendering
            canvas.set_draw_color(Color::RGB(150, 64, 105));
            canvas.clear();
            for event in event_pump.poll_iter() {
                println!("{:?}", event);
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            canvas.present();
            //manually updating and rendering end
            update(); // capping updates
            update_counter += 1;
            render(); // capping fps
            fps_counter += 1;

            //tick the clock once (probably should go in update)
            clock += 1;
            if clock >= FPS {
                clock = 0;
            }
            delta -= 1.0;
        }
        // update();            // uncapping updates
        // update_counter += 1;
        // render();            // uncapping fps
        // fps_counter += 1;
        if SystemTime::now().duration_since(timer).unwrap().as_secs() > 1 {
            timer += Duration::new(1, 0);
            canvas // showing fps in title
                .window_mut()
                .set_title(
                    format!(
                        "{} | Updates: {} | FPS: {}",
                        TITLE, update_counter, fps_counter
                    )
                    .as_str(),
                )
                .unwrap();
            update_counter = 0;
            fps_counter = 0;
        }
    }
}

fn update() {}
fn render() {}
