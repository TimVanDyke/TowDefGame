use gl::types::*;

pub mod VertexBuffer

pub struct VertexBuffer {
    m_renderer_id: GLuint,
}

impl VertexBuffer {
    // pub fn new(data: *const GLvoid, size: GLuint) -> VertexBuffer {
    //     gl::GenBuffers(1, &m_renderer_id);
    // }
    pub fn new() -> VertexBuffer {
        VertexBuffer {
            
        }
    }
}