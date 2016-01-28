extern crate gl;
use super::super::Renderer;

pub struct OpenGLRenderer;

impl OpenGLRenderer {
    pub fn new() -> Box<OpenGLRenderer> {
        Box::new(OpenGLRenderer)
    }
}

impl Renderer for OpenGLRenderer {
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

