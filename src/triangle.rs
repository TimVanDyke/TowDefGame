use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
use failure;
use gl;
use crate::Render;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Tri1 {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
}

pub struct Tri2 {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
}

impl Tri1 {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Tri1, failure::Error> {
        // set up shader program

        let program = render_gl::Program::from_res(gl, res, "shaders/tri")?;

        // set up vertex buffer object

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (0.0, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (-1.0, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            }, // top
        ];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Tri1 {
            program,
            _vbo: vbo,
            vao,
        })
    }
}

impl Tri2 {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Tri2, failure::Error> {
        // set up shader program

        let program = render_gl::Program::from_res(gl, res, "shaders/tri")?;

        // set up vertex buffer object

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (1.0, -0.5, 0.0).into(),
                clr: (0.0, 180.0 / 255.0, 180.0 / 255.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (0.0, -0.5, 0.0).into(),
                clr: (1.0, 102.0 / 255.0, 178.0 / 255.0, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (1.0, 1.0, 133.0 / 255.0, 1.0).into(),
            }, // top
        ];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Tri2 {
            program,
            _vbo: vbo,
            vao,
        })
    }
}
impl Render for Tri1 {
    fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }
        self.vao.unbind();
    }
}
impl Render for Tri2 {
    fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }
        self.vao.unbind();
    }
}
