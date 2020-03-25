use crate::resources::Resources;
use failure;
use gl;
use std::os::raw;

pub struct TextureLoadOptions<'a> {
    resource_name: &'a str,
    format: gl::types::GLenum,
    pub gen_mipmaps: bool,
}

impl<'a> TextureLoadOptions<'a> {
    pub fn from_res_rgb(resource_name: &str) -> TextureLoadOptions {
        TextureLoadOptions {
            resource_name,
            format: gl::RGB,
            gen_mipmaps: false,
        }
    }

    pub fn from_res_rgba(resource_name: &str) -> TextureLoadOptions {
        TextureLoadOptions {
            resource_name,
            format: gl::RGBA,
            gen_mipmaps: false,
        }
    }
}

pub struct TextureLoadBuilder<'a> {
    options: TextureLoadOptions<'a>,
}

impl<'a> TextureLoadBuilder<'a> {
    pub fn load(self, gl: &gl::Gl, res: &Resources) -> Result<Texture, failure::Error> {
        Texture::from_res(self.options, gl, res)
    }

    pub fn with_gen_mipmaps(mut self) -> Self {
        self.options.gen_mipmaps = true;
        self
    }
}

pub struct Texture {
    gl: gl::Gl,
    obj: gl::types::GLuint,
}

impl Texture {
    pub fn from_res_rgb(resource_name: &str) -> TextureLoadBuilder {
        TextureLoadBuilder {
            options: TextureLoadOptions::from_res_rgb(resource_name),
        }
    }

    pub fn from_res_rgba(resource_name: &str) -> TextureLoadBuilder {
        TextureLoadBuilder {
            options: TextureLoadOptions::from_res_rgba(resource_name),
        }
    }

    pub fn from_res<'a>(
        options: TextureLoadOptions<'a>,
        gl: &gl::Gl,
        res: &Resources,
    ) -> Result<Texture, failure::Error> {
        let mut obj: gl::types::GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut obj);
        }

        let texture = Texture {
            gl: gl.clone(),
            obj,
        };

        texture.update(options, res)?;

        Ok(texture)
    }

    pub fn update<'a>(
        &self,
        options: TextureLoadOptions<'a>,
        res: &Resources,
    ) -> Result<(), failure::Error> {
        let gl = &self.gl;

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, self.obj);
        }

        // https://www.khronos.org/opengl/wiki/Common_Mistakes

        match options.format {
            gl::RGB => {
                let img = res.load_rgb_image(options.resource_name)?;

                if options.gen_mipmaps {
                    unsafe {
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGB8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                        gl.GenerateMipmap(gl::TEXTURE_2D);
                    }
                } else {
                    unsafe {
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGB8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                    }
                }
            }
            gl::RGBA => {
                let img = res.load_rgba_image(options.resource_name)?;

                if options.gen_mipmaps {
                    unsafe {
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGBA8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                        gl.GenerateMipmap(gl::TEXTURE_2D);
                    }
                } else {
                    unsafe {
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGBA8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                    }
                }
            }
            _ => unreachable!("Only RGB or RGBA images can be constructed"),
        }

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.obj);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind_at(&self, index: u32) {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0 + index);
        }
        self.bind();
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, &mut self.obj) };
    }
}

// My first NON RUSTY attempt, following my man nercury now

// use crate::resources::Resources;
// use gl::*;
// use std::string::String;

// struct Texture {
//     renderer_id: gl::types::GLuint, // unsigned int
//     file_path: String,              // std::string
//     img: image::RgbaImage,          // char*
//     width: i32,                     // int
//     height: i32,                    // int
//     bpp: i32,                       // int
// }

// impl Texture {
//     pub fn new(res: &Resources, name: String, gl: &gl::Gl) -> Result<Texture, failure::Error> {
//         let mut renderer_id: gl::types::GLuint = 0;
//         let file_path = name;
//         let mut img = res.load_rgba_image(&file_path)?;
//         let mut width = img.width() as i32;
//         let mut height = img.height() as i32;
//         let mut bpp = 0;

//         unsafe {
//             gl.GenTextures(1, &mut renderer_id);
//             gl.BindTexture(TEXTURE_2D, renderer_id);

//             gl.TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as types::GLint);
//             gl.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as types::GLint);
//             gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as types::GLint);
//             gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as types::GLint);

//             gl.TexImage2D(
//                 TEXTURE_2D,
//                 0,
//                 RGBA8 as types::GLint,
//                 width,
//                 width,
//                 0,
//                 RGBA,
//                 UNSIGNED_BYTE,
//                 img.as_ptr() as *const std::os::raw::c_void,
//             )
//         }

//         Ok(Texture {
//             renderer_id,
//             file_path,
//             img,
//             width,
//             height,
//             bpp,
//         })
//     }
//     pub fn Bind(&self, gl: &gl::Gl) {}
//     pub fn Unbind(&self, slot: gl::types::GLuint, gl: &gl::Gl) {}
//     pub fn get_width(&self) -> i32 {
//         self.width
//     }
//     pub fn get_height(&self) -> i32 {
//         self.height
//     }
// }

// impl Drop for Texture {
//     fn drop(&mut self) {
//         unsafe { self.gl.DeleteTextures(1, &mut self.obj) };
//     }
// }
