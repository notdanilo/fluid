use gpu::{Context, ContextBuilder, ContextDisplay, ClearProgram, Framebuffer, FragmentShader, VertexShader, RasterProgram, Buffer, VertexArrayObject, RasterGeometry};
use std::mem::size_of;
use crate::math::DIMENSIONS;
use crate::particles::Particles;

pub struct Renderer {
    context: Context,
    clear: ClearProgram,
    raster: RasterProgram,
    vao: VertexArrayObject,
    framebuffer: Framebuffer,
    positions: Buffer
}

impl Renderer {
    pub fn new(count: usize) -> Self {
        let context = ContextBuilder::new().with_display(ContextDisplay::Screen).build();
        context.make_current().ok();

        let clear = ClearProgram::new(&context);
        let framebuffer = Framebuffer::default(&context);

        let fragment = FragmentShader::new(&context, include_str!("fragment.glsl")).expect("Couldn't compile fragment shader.");
        let vertex = VertexShader::new(&context, include_str!("vertex.glsl")).expect("Couldn't compile vertex shader.");
        let raster = RasterProgram::new(&context, &fragment, &vertex).expect("Couldn't link program.");

        let positions = Buffer::allocate(&context, size_of::<f32>() * DIMENSIONS * count);

        let mut vao = VertexArrayObject::new(&context);
        vao.set_vertex_buffer(&positions, 0, DIMENSIONS as u32);
        Self { context, clear, raster, vao, framebuffer, positions }
    }

    pub fn up(&mut self) -> bool {
        self.context.run()
    }

    fn upload(&mut self, particles: &Particles) {
        let mut data = Vec::new();
        for particle in &particles.particles {
            data.push(particle.position.x);
            data.push(particle.position.y);
            data.push(particle.position.z);
        }
        self.positions.set_data(&data);
    }

    pub fn render(&mut self, particles: &Particles) {
        self.upload(particles);
        self.clear.clear(&mut self.framebuffer, ClearProgram::COLOR);
        self.raster.uniform_mat4(0, false, nalgebra::Orthographic3::new(0.0, 1920.0, 0.0, 1080.0, 0.0, 1.0).as_matrix().as_slice());
        self.raster.raster(&self.framebuffer, &self.vao, RasterGeometry::Points, particles.count() as u32);
        self.context.swap_buffers().ok();
    }
}