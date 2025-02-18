use gl::types::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::collections::HashMap;
use std::ptr;
use std::time::Instant;
use cgmath::{Matrix4, Rad, PerspectiveFov, Deg, Vector3};
use cgmath::prelude::*;

mod shader;
mod texture_manager;
mod numbers;
mod block_type;

use shader::Shader;
use block_type::BlockType;
use texture_manager::TextureManager;
use numbers::Numbers;

#[allow(dead_code)]
struct Window {
    width: i32,
    height: i32,
    vao: GLuint,
    vertex_position_vbo: GLuint,
    tex_coord_vbo: GLuint,
    ibo: GLuint,
    shader: Shader,
    shader_matrix_location: i32,
    shader_sampler_location: i32,
    mv_matrix: Matrix4<f32>,
    p_matrix: Matrix4<f32>,
    x: f32,
    last_time: Instant,
    texture_manager: TextureManager,
    blocks: HashMap<String, BlockType>
}

impl Window {
    fn new(width: i32, height: i32) -> Self {
        // create blocks
        let numbers = Numbers::new();
        let mut texture_manager = TextureManager::new(16, 16, 256);
        let mut blocks = HashMap::new();

        blocks.insert(
            String::from("cobblestone"),
            BlockType::new(
                &mut texture_manager, 
                "cobblestone", 
                HashMap::from([
                    (String::from("all"), String::from("cobblestone"))
                ]), 
                &numbers
            )
        );
        blocks.insert(
            String::from("grass"),
            BlockType::new(
                &mut texture_manager,
                "grass",
                HashMap::from([
                    (String::from("top"), String::from("grass")),
                    (String::from("bottom"), String::from("dirt")),
                    (String::from("sides"), String::from("grass_side"))
                ]),
                &numbers
            )
        );
        blocks.insert(
            String::from("dirt"),
            BlockType::new(
                &mut texture_manager,
                "dirt",
                HashMap::from([
                    (String::from("all"), String::from("dirt"))
                ]),
                &numbers
            )
        );
        blocks.insert(
            String::from("stone"),
            BlockType::new(
                &mut texture_manager,
                "stone",
                HashMap::from([
                    (String::from("all"), String::from("stone"))
                ]),
                &numbers
            )
        );
        blocks.insert(
            String::from("sand"),
            BlockType::new(
                &mut texture_manager,
                "sand",
                HashMap::from([
                    (String::from("all"), String::from("sand"))
                ]),
                &numbers
            )
        );
        blocks.insert(
            String::from("planks"),
            BlockType::new(
                &mut texture_manager,
                "planks",
                HashMap::from([
                    (String::from("all"), String::from("planks"))
                ]),
                &numbers
            )
        );
        blocks.insert(
            String::from("log"),
            BlockType::new(
                &mut texture_manager,
                "log",
                HashMap::from([
                    (String::from("top"), String::from("log_top")),
                    (String::from("bottom"), String::from("log_top")),
                    (String::from("sides"), String::from("log_side"))
                ]),
                &numbers
            )
        );

        texture_manager.generate_mipmaps();
        
        let mut vao = 0;
        let mut vertex_position_vbo = 0;
        let mut tex_coord_vbo = 0;
        let mut ibo = 0;

        unsafe {
            // create vertex array object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // create vertex position vbo
            gl::GenBuffers(1, &mut vertex_position_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_position_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (blocks.get_mut("grass").unwrap().vertex_positions.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                blocks.get_mut("grass").unwrap().vertex_positions.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(0);

            // create tex coord vbo
            gl::GenBuffers(1, &mut tex_coord_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, tex_coord_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (blocks.get_mut("grass").unwrap().tex_coords.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                blocks.get_mut("grass").unwrap().tex_coords.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(1);

            // create index buffer object
            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (blocks.get_mut("grass").unwrap().indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                blocks.get_mut("grass").unwrap().indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }

        // create shader
        let shader = Shader::new("shaders/vert.glsl", "shaders/frag.glsl").unwrap();
        let shader_matrix_location = shader.find_uniform("matrix");
        let shader_sampler_location = shader.find_uniform("texture_array_sampler");
        shader.use_program();

        let mv_matrix = Matrix4::identity();
        let p_matrix = Matrix4::identity();

        Self { 
            width, height, 
            vao, vertex_position_vbo, tex_coord_vbo, ibo, 
            shader, shader_matrix_location, shader_sampler_location, 
            mv_matrix, p_matrix, 
            x: 0.0, last_time: Instant::now(), 
            texture_manager, blocks 
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.x += delta_time;
    }

    fn draw(&mut self) {
        unsafe {
            // create projection matrix
            self.p_matrix = Matrix4::from(PerspectiveFov {
                fovy: Deg(90.0).into(),
                aspect: self.width as f32 / self.height as f32,
                near: 0.1,
                far: 500.0,
            });

            // create model view matrix
            self.mv_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0));
            self.mv_matrix = self.mv_matrix * Matrix4::from_angle_x(Rad(self.x));
            self.mv_matrix = self.mv_matrix * Matrix4::from_angle_y(Rad(((self.x / 3.0 * 2.0) / 2.0).sin()));

            // multiply the two matrices together and send to the shader program
            let mvp_matrix = self.p_matrix * self.mv_matrix;
            self.shader.uniform_matrix(self.shader_matrix_location, &mvp_matrix);

            // bind textures
            // Set our active texture unit to the first texture unit
            gl::ActiveTexture(gl::TEXTURE0);

            // Bind our texture manager's texture
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.texture_manager.texture_array);

            // tell our sampler our texture is bound to the first texture unit
            gl::Uniform1i(self.shader_sampler_location, 0);

            // draw stuff
            gl::Enable(gl::DEPTH_TEST); // enable depth testing so faces are drawn in the right order
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, self.blocks.get("grass").unwrap().indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Voxel Engine")
        .with_inner_size(glutin::dpi::LogicalSize::new(800, 600));

    let context = ContextBuilder::new()
        .with_vsync(false)
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let context = unsafe { context.make_current().unwrap() };

    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let mut window = Window::new(800, 600);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    unsafe { gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32) };
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = (now - window.last_time).as_secs_f32();
                window.last_time = now;

                window.update(delta_time);
                window.draw();
                context.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}
