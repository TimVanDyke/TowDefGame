#[macro_use]
extern crate failure;
#[macro_use]
extern crate render_gl_derive;

pub mod camera;
mod debug;
pub mod render_gl;
pub mod resources;
mod textured_square;

use crate::resources::Resources;
use failure::err_msg;
use nalgebra as na;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::time::{Duration, SystemTime};

trait Render {
    fn render(&self, gl: &gl::Gl);
}

trait RenderTex {
    fn render(&self, gl: &gl::Gl, view_matrix: &na::Matrix4<f32>, proj_matrix: &na::Matrix4<f32>);
}

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
    let mut lst_time = SystemTime::now();
    let mut delta: f64 = 0.0;
    let mut clk: u8 = 0;
    let mut fps_cnt = 0;
    let mut updt_cnt = 0;
    let mut running = true;

    let res = Resources::from_relative_exe_path(Path::new("res")).unwrap();

    let sdl = sdl2::init().map_err(err_msg)?;
    let vid_sub = sdl.video().map_err(err_msg)?;

    let gl_attr = vid_sub.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let win_size: (i32, i32) = (900, 700);

    let mut window = vid_sub
        .window(TITLE, win_size.0 as u32, win_size.1 as u32)
        .position_centered()
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| vid_sub.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut viewprt = render_gl::Viewport::for_window(win_size.0, win_size.1);
    let clr_bffr = render_gl::ColorBuffer::new();

    let cam = camera::Camera::new(1.0, 1.0);

    let tex = textured_square::TexturedSquare::new(&res, &gl)?;

    // let gmeobjs: Vec<&dyn Update> = vec![some_updatable_obj];
    let drawables: Vec<&dyn RenderTex> = vec![&tex];

    // set up shared state for window
    viewprt.set_used(&gl);
    clr_bffr.set_clear_color(&gl, na::Vector3::new(0.0, 0.0, 0.0));
    let mut pump = sdl.event_pump().map_err(err_msg)?;
    // game loop
    while running {
        // fps and update timer goes here
        time_now = SystemTime::now();
        delta += time_now.duration_since(lst_time).unwrap().as_nanos() as f64 / NANOS;
        lst_time = time_now;
        // capping updates to "UPDATES"
        while delta > 1.0 {
            running = handle_events(
                &mut pump,
                &mut updt_cnt,
                &mut clk,
                UPDATES,
                &mut viewprt,
                &gl,
            );
            render(&mut window, &cam, &clr_bffr, &drawables, &mut fps_cnt, &gl);
            delta -= 1.0;
        } // uncapping updates and fps is below:
        build_title_update_fps(&mut timer, &mut window, TITLE, &mut updt_cnt, &mut fps_cnt);
    }
    Ok(())
}

/// handles user input and events, then updates the world based on those things
fn handle_events(
    pump: &mut sdl2::EventPump,
    updt_cnt: &mut i32,
    clk: &mut u8,
    updates: u8,
    viewprt: &mut render_gl::Viewport,
    gl: &gl::Gl,
) -> bool {
    for event in pump.poll_iter() {
        println!("{:?}", event);
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(w, h),
                ..
            } => {
                viewprt.update_size(w, h);
                viewprt.set_used(&gl);
            }
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
    cam: &camera::Camera,
    clr_bffr: &render_gl::ColorBuffer,
    drawables: &Vec<&dyn RenderTex>,
    fps_cnt: &mut i32,
    gl: &gl::Gl,
) {
    clr_bffr.clear(&gl);
    for drawable in drawables.iter() {
        drawable.render(&gl, &cam.get_view_matrix(), &cam.get_p_matrix());
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
