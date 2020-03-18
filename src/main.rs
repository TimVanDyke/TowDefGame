#[macro_use]
extern crate failure;
#[macro_use]
extern crate render_gl_derive as render_gl_derive;

mod debug;
pub mod render_gl;
pub mod resources;
mod square;
mod triangle;

use crate::resources::Resources;
use failure::err_msg;
use nalgebra as na;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::time::{Duration, SystemTime};

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

/// Initialized variables and begins the game loop.
/// It also calls the helper methods within the loop to keep the loop clean.
fn run() -> Result<(), failure::Error> {
    const TITLE: &str = "Protect Joe";

    // fps Calc and Game Clock
    const UPDATES: u8 = 60;
    const NANOS: f64 = 1_000_000_000.0 / UPDATES as f64;
    let mut time_now: SystemTime;
    let mut timer = SystemTime::now();
    let mut last_time = SystemTime::now();
    let mut delta: f64 = 0.0;
    let mut clock: u8 = 0;
    let mut fps_count = 0;
    let mut update_count = 0;
    let mut running = true;

    let res = Resources::from_relative_exe_path(Path::new("res")).unwrap();

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let mut window = video_subsystem
        .window(TITLE, 900, 700)
        .position_centered()
        .opengl()
        .resizable()
        .build()?;

    let mut i = 0;
    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut viewport = render_gl::Viewport::for_window(900, 700);
    let color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    let sqr = square::Square::new(&res, &gl)?;
    let tri1 = triangle::Tri1::new(&res, &gl)?;
    let tri2 = triangle::Tri2::new(&res, &gl)?;
    let drawables = vec![sqr];

    // set up shared state for window

    viewport.set_used(&gl);
    color_buffer.set_used(&gl);

    // main loop

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    // game loop
    while running {
        // fps and update timer goes here
        time_now = SystemTime::now();
        delta += time_now.duration_since(last_time).unwrap().as_nanos() as f64 / NANOS;
        last_time = time_now;
        // capping updates to "UPDATES"
        while delta > 1.0 {
            running = handle_events(&mut event_pump, &mut update_count, &mut clock, UPDATES);
            render(&mut window, &drawables, &mut fps_count, &gl);
            delta -= 1.0;
        } // uncapping updates and fps is below:
          // running = handle_events(&mut event_pump, &mut update_count, &mut clock, UPDATES);
          // render(&mut window, &mut fps_count);
        build_title_update_fps(
            &mut timer,
            &mut window,
            TITLE,
            &mut update_count,
            &mut fps_count,
        );
    }
    Ok(())
}

/// handles user input and events, then updates the world based on those things
fn handle_events(
    pump: &mut sdl2::EventPump,
    updt_cnt: &mut i32,
    clk: &mut u8,
    updates: u8,
) -> bool {
    for event in pump.poll_iter() {
        println!("{:?}", event);
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {}
        }
    }
    // tick the clock once
    *clk += 1;
    if *clk >= updates {
        *clk = 0;
    }
    *updt_cnt += 1;
    true
}

/// renders the world to handle what happened in handle_events
fn render(
    window: &mut sdl2::video::Window,
    drawables: &Vec<square::Square>,
    fps_cnt: &mut i32,
    gl: &gl::Gl,
) {
    for drawable in drawables {
        drawable.render(&gl);
    }
    window.gl_swap_window();
    *fps_cnt += 1;
}

/// A helper method to build the title for the window so that it doesn't look like garbage in my loop
fn build_title_update_fps(
    timer: &mut SystemTime,
    window: &mut sdl2::video::Window,
    title: &str,
    updt_cnt: &mut i32,
    fps_cnt: &mut i32,
) {
    if SystemTime::now().duration_since(*timer).unwrap().as_secs() > 1 {
        *timer += Duration::new(1, 0);
        window // showing fps in title
            .set_title(
                format!(
                    "{} | Updates: {} | FPS: {}",
                    title, updt_cnt, fps_cnt
                )
                .as_str(),
            )
            .unwrap();
        *updt_cnt = 0;
        *fps_cnt = 0;
    }
}
