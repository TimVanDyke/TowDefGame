use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
use crate::RenderTex;

use failure;
use gl;
use na::Matrix3;
use nalgebra as na;
#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
    #[location = "2"]
    normal: data::f32_f32_f32,
    #[location = "3"]
    uv: data::f16_f16,
}

pub struct TexturedSquare {
    program: render_gl::Program,
    texture: render_gl::Texture,
    position: Matrix3<f32>,
    program_view_location: Option<i32>,
    program_projection_location: Option<i32>,
    camera_pos_location: Option<i32>,
    tex_face_location: Option<i32>,
    _vbo: buffer::ArrayBuffer,
    _ibo: buffer::ElementArrayBuffer,
    index_count: i32,
    vao: buffer::VertexArray,
    _debug_rays: Vec<render_gl::RayMarker>,
}
impl TexturedSquare {
    pub fn new(
        res: &Resources,
        gl: &gl::Gl,
        debug_lines: &render_gl::DebugLines,
    ) -> Result<TexturedSquare, failure::Error> {
        // set up shader program
        let texture = render_gl::Texture::from_res_rgb("textures/test.png").load(gl, res)?;
        let program = render_gl::Program::from_res(gl, res, "shaders/textured/tex")?;

        let program_view_location = program.get_uniform_location("View");
        let program_projection_location = program.get_uniform_location("Projection");
        let camera_pos_location = program.get_uniform_location("CameraPos");
        let tex_face_location = program.get_uniform_location("TexFace");

        // set up vertex buffer object
        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
                uv: (0.0, 0.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
                uv: (1.0, 0.0).into(),
            }, // bottom right
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
                uv: (1.0, 1.0).into(),
            }, // top right
            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                clr: (1.0, 1.0, 0.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
                uv: (0.0, 1.0).into(),
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
            program_view_location,
            program_projection_location,
            camera_pos_location,
            tex_face_location,
            position: Matrix3::identity(),
            _vbo: vbo,
            _ibo: ibo,
            index_count: indices.len() as i32,
            vao,
            _debug_rays: vertices
                .iter()
                .map(|v| {
                    debug_lines.ray_marker(
                        na::Point3::new(v.pos.d0, v.pos.d1, v.pos.d2),
                        na::Vector3::new(v.normal.d0, v.normal.d1, v.normal.d2),
                        na::Vector4::new(
                            v.clr.inner.x(),
                            v.clr.inner.y(),
                            v.clr.inner.z(),
                            v.clr.inner.w(),
                        ),
                    )
                })
                .collect(),
        })
    }
}

impl RenderTex for TexturedSquare {
    fn render(
        &self,
        gl: &gl::Gl,
        view_matrix: &na::Matrix4<f32>,
        proj_matrix: &na::Matrix4<f32>,
        camera_pos: &na::Vector3<f32>,
    ) {
        // set shader
        self.program.set_used();

        if let Some(loc) = self.tex_face_location {
            self.texture.bind_at(0);
            self.program.set_uniform_1i(loc, 0);
        }
        if let Some(loc) = self.program_view_location {
            self.program.set_uniform_matrix_4fv(loc, view_matrix);
        }
        if let Some(loc) = self.program_projection_location {
            self.program.set_uniform_matrix_4fv(loc, proj_matrix);
        }
        if let Some(loc) = self.camera_pos_location {
            self.program.set_uniform_3f(loc, camera_pos);
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
