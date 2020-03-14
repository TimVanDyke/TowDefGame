use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::{Duration, SystemTime};

pub fn main() {
    // fps Calc and Game Clock
    const FPS: u8 = 60;
    const NANOS: f64 = 1_000_000_000.0 / FPS as f64;
    let mut time_now: SystemTime;
    let mut timer = SystemTime::now();
    let mut last_time = SystemTime::now();
    let mut delta: f64 = 0.0;
    let mut clock: u8 = 0;
    let mut fps_count = 0;
    let mut update_count = 0;

    // sdl setup and window setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    const TITLE: &str = "Protect Jo";
    let mut window = video_subsystem
        .window(TITLE, 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // graphics context and opengl setup
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // game loop
    'main: loop {
        // fps and update timer goes here
        time_now = SystemTime::now();
        delta += time_now.duration_since(last_time).unwrap().as_nanos() as f64 / NANOS;
        last_time = time_now;
        while delta > 1.0 {
            for event in event_pump.poll_iter() {
                println!("{:?}", event);
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    _ => {}
                }
            }
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            window.gl_swap_window();
            //manually updating and rendering end
            update(); // capping updates
            update_count += 1;
            render(); // capping fps
            fps_count += 1;

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
        build_title_update_fps(
            &mut timer,
            &mut window,
            TITLE,
            &mut update_count,
            &mut fps_count,
        );
    }
}

fn update() {}
fn render() {}

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
