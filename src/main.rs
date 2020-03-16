#![allow(unused_imports)]
extern crate gl;
use gl::types::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};

pub fn main() {
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
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5];
    let indices: Vec<gl::types::GLuint> = vec![0, 1, 2, 2, 3, 0];
    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    let mut ibo: gl::types::GLuint = 0;
    unsafe {
        // instantiate stuff with vertices (vbo)
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        println!("{}", *gl::GetString(gl::VENDOR));
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // instantiate array stuff (vao)
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // instantiate stuff with indices buffer
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (std::mem::size_of::<f32>() * 2) as gl::types::GLsizei,
            0 as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(0);
    }

    // game loop
    while running {
        // fps and update timer goes here
        time_now = SystemTime::now();
        delta += time_now.duration_since(last_time).unwrap().as_nanos() as f64 / NANOS;
        last_time = time_now;
        // capping updates to "UPDATES"
        while delta > 1.0 {
            running = handle_events(&mut event_pump, &mut update_count, &mut clock, UPDATES);
            render(&mut window, &mut fps_count);
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
}

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

fn render(window: &mut sdl2::video::Window, fps_cnt: &mut i32) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        // gl::DrawArrays(gl::TRIANGLES, 0, 4);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
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

// fn create_shader(vertex_shader: &str, fragment_shader: &str) -> i32 {
//     unsafe {
//         let program: u32 = gl::CreateProgram();
//         let vs: u32 = gl::CreateShader(gl::VERTEX_SHADER);
//     }
//     4
// }

// fn compile_shader(shader_type: GLuint, source: &str) {
//     unsafe {
//         let id: GLuint = gl::CreateShader(shader_type);
//     }
// }
