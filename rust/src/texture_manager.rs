use gl::types::*;
use std::collections::HashMap;
use std::ptr;

pub struct TextureManager {
    texture_width: i32,
    texture_height: i32,
    max_textures: i32,
    pub texture_array: GLuint,
    pub textures: HashMap<String, i32>,
}

impl TextureManager {
    pub fn new(texture_width: i32, texture_height: i32, max_textures: i32) -> Self {
        let mut texture_array = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_array);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, texture_array);

            // disable texture filtering for magnification (return the texel that's nearest to the fragment's texture coordinate)
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA as GLint,
                texture_width,
                texture_height,
                max_textures,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null()
            );
        }

        Self {
            texture_width,
            texture_height,
            max_textures,
            texture_array,
            textures: HashMap::new()
        }
    }

    pub fn generate_mipmaps(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.texture_array);
            gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        }
    }

    pub fn add_texture(&mut self, texture_name: &str) {
        if self.textures.contains_key(texture_name) {
            return;
        }

        let texture_index = self.textures.len() as i32;
        if texture_index >= self.max_textures {
            eprintln!("Maximum number of textures reached !");
            return;
        }

        let image = image::open(format!("../textures/{}.png", texture_name)).expect("Failed to load texture");
        // let image = image.flipv(); // Flip vertically if necessary
        let data = image.to_rgb8().into_raw();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.texture_array);
            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                0,
                0,
                texture_index,
                self.texture_width,
                self.texture_height,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _
            );
        }

        self.textures.insert(texture_name.to_string(), texture_index);
    }

    pub fn get_texture_index(&self, texture: &str) -> Option<usize> {
        self.textures.iter().position(|t| t.0 == texture)
    }
}
