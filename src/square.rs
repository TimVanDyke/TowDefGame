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

pub struct Square {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    _ibo: buffer::ElementArrayBuffer,
    vao: buffer::VertexArray,
}
impl Square {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Square, failure::Error> {
        // set up shader program

        let program = render_gl::Program::from_res(gl, res, "shaders/tri")?;

        // set up vertex buffer object

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            }, // top right
            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                clr: (1.0, 1.0, 1.0, 1.0).into(),
            }, // top left
        ];
        let indices: Vec<gl::types::GLuint> = vec![0, 1, 2, 2, 3, 0];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        // vbo.unbind();
        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);
        vao.bind();
        // set up indices array object

        let ibo = buffer::ElementArrayBuffer::new(gl);
        ibo.bind();
        ibo.static_draw_data(&indices);
        // ibo.unbind();

        // vao.bind();
        // vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();
        ibo.unbind();

        Ok(Square {
            program,
            _vbo: vbo,
            _ibo: ibo,
            vao,
        })
    }
}

impl Render for Square {
    fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,    // mode
                6,                // starting index in the enabled arrays
                gl::UNSIGNED_INT, // number of indices to be rendered
                std::ptr::null(),
            );
        }
        self.vao.unbind();
    }
}
