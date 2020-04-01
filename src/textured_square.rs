use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
use crate::Render;
use crate::Update;

use failure;
use gl;
use nalgebra as na;
#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    tc: data::f16_f16,
}

pub struct TexturedSquare {
    program: render_gl::Program,
    texture: render_gl::Texture,
    position: na::Vector3<f32>,
    program_model_location: Option<i32>,
    program_projection_location: Option<i32>,
    tex_face_location: Option<i32>,
    _vbo: buffer::ArrayBuffer,
    _ibo: buffer::ElementArrayBuffer,
    index_count: i32,
    vao: buffer::VertexArray,
}
impl TexturedSquare {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<TexturedSquare, failure::Error> {
        // set up shader program
        let texture = render_gl::Texture::from_res_rgb("textures/test.png").load(gl, res)?;
        let program = render_gl::Program::from_res(gl, res, "shaders/tex")?;

        let program_model_location = program.get_uniform_location("Model");
        let program_projection_location = program.get_uniform_location("Projection");
        let tex_face_location = program.get_uniform_location("TexFace");

        // set up vertex buffer object
        let vertices: Vec<Vertex> = vec![
            // uv coords intentionally backwards so that it renders right
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                tc: (1.0, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                tc: (0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                tc: (0.0, 0.0).into(),
            }, // top right
            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                tc: (1.0, 0.0).into(),
            }, // top left
        ];
        let indices: Vec<gl::types::GLuint> = vec![0, 1, 2, 2, 3, 0];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);

        // set up indices array object
        let ibo = buffer::ElementArrayBuffer::new(gl);
        ibo.bind();
        ibo.static_draw_data(&indices);

        // set up vertex array object
        let vao = buffer::VertexArray::new(gl);
        vao.bind();
        vbo.bind();
        ibo.bind();

        Vertex::vertex_attrib_pointers(gl);

        Ok(TexturedSquare {
            program,
            texture,
            program_model_location,
            program_projection_location,
            tex_face_location,
            position: na::Vector3::new(0.0, 0.0, 0.0),
            _vbo: vbo,
            _ibo: ibo,
            index_count: indices.len() as i32,
            vao,
        })
    }

    fn update(&mut self) {
        self.position[0] += 0.01;
        self.position[1] += 0.01;
    }
}

impl Update for TexturedSquare {
    fn update(&mut self) {
        self.update();
        println!("{}", self.position);
    }
}
impl Render for TexturedSquare {
    fn render(&self, gl: &gl::Gl, proj_matrix: &na::Matrix4<f32>) {
        // set shader
        self.program.set_used();

        if let Some(loc) = self.program_model_location {
            self.program.set_uniform_matrix_4fv(
                loc,
                &na::geometry::Translation3::new(
                    self.position[0],
                    self.position[1],
                    self.position[2],
                )
                .to_homogeneous(),
            );
        }
        if let Some(loc) = self.program_projection_location {
            self.program.set_uniform_matrix_4fv(loc, proj_matrix);
        }
        if let Some(loc) = self.tex_face_location {
            self.texture.bind_at(0);
            self.program.set_uniform_1i(loc, 0);
        }
        self.vao.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
