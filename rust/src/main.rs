use gl::types::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::ptr;

mod shader;
mod matrix;

use shader::Shader;
use matrix::Matrix;

const VERTEX_POSITIONS: [f32; 12] = [
    -0.5, 0.5, 0.0,  // Top left
    -0.5, -0.5, 0.0, // Bottom left
    0.5, -0.5, 0.0,  // Bottom right
    0.5, 0.5, 0.0,   // Top right
];

const INDICES: [u32; 6] = [
    0, 1, 2, // First triangle
    0, 2, 3, // Second triangle
];

#[allow(dead_code)]
struct Window {
    width: i32,
    height: i32,
    vao: GLuint,
    vbo: GLuint,
    ibo: GLuint,
    shader: Shader,
    shader_matrix_location: i32,
    mv_matrix: Matrix,
    p_matrix: Matrix,
    x: f32
}

impl Window {
    fn new(width: i32, height: i32) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ibo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_POSITIONS.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                VERTEX_POSITIONS.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (INDICES.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                INDICES.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }

        let shader = Shader::new("shaders/vert.glsl", "shaders/frag.glsl").unwrap();
        let shader_matrix_location = shader.find_uniform("matrix");
        shader.use_program();

        let mv_matrix = Matrix::new();
        let p_matrix = Matrix::new();

        Self { width, height, vao, vbo, ibo, shader, shader_matrix_location, mv_matrix, p_matrix, x: 0.0 }
    }

    fn update(&mut self, delta_time: f32) {
        self.x += delta_time;
    }

    fn draw(&mut self) {
        unsafe {
            // create projection matrix
            self.p_matrix.load_identity();
            self.p_matrix.perspective(90.0, (self.width / self.height) as f64, 0.1, 500.0);

            // create model view matrix
            self.mv_matrix.load_identity();
            self.mv_matrix.translate(0.0, 0.0, -1.0);
            self.mv_matrix.rotate_2d((self.x + 6.28 / 4.0).into(), ((self.x / 3.0 * 2.0).sin() / 2.0).into());

            // multiply the two matrices together and send to the shader program
            let mvp_matrix = self.p_matrix.multiply(&self.mv_matrix);
            self.shader.uniform_matrix(self.shader_matrix_location, &mvp_matrix);

            // draw stuff
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, INDICES.len() as i32, gl::UNSIGNED_INT, ptr::null());
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
            Event::RedrawRequested(_) => {
                window.update(0.016);
                window.draw();
                context.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}