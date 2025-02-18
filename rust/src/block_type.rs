use std::collections::HashMap;
use crate::texture_manager::TextureManager;
use crate::numbers::Numbers;

#[allow(dead_code)]
pub struct BlockType {
    pub name: String,
    pub vertex_positions: Vec<f32>,
    pub tex_coords: Vec<f32>,
    pub indices: Vec<u32>
}

impl BlockType {
    pub fn new(texture_manager: &mut TextureManager, name: &str, block_face_textures: HashMap<String, String>, numbers: &Numbers) -> Self {
        let mut block_type = BlockType {
            name: name.to_string(),
            vertex_positions: numbers.vertex_position.clone(),
            tex_coords: numbers.tex_coords.clone(),
            indices: numbers.indices.clone()
        };

        // set a specific face of the block to a certain texture
        fn set_block_face(tex_coords: &mut Vec<f32>, face: usize, texture_index: usize) {
            for vertex in 0..4 {
                tex_coords[face * 12 + vertex * 3 + 2] = texture_index as f32;
            }
        }

        for (face, texture) in block_face_textures {
            texture_manager.add_texture(&texture);

            let texture_index = texture_manager.get_texture_index(&texture).unwrap();

            if face == "all" {
                for i in 0..6 {
                    set_block_face(&mut block_type.tex_coords, i, texture_index);
                }
            }
            else if face == "sides" {
                let sides = [0, 1, 4, 5];
                for &i in &sides {
                    set_block_face(&mut block_type.tex_coords, i, texture_index);
                }
            }
            else {
                let faces = ["right", "left", "top", "bottom", "front", "back"];
                if let Some(index) = faces.iter().position(|&f| f == face) {
                    set_block_face(&mut block_type.tex_coords, index, texture_index);
                }
            }
        }

        block_type
    }
}
