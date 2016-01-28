use super::Renderer;
use self::opengl::OpenGLRenderer;

pub mod opengl;

pub fn determine_best_renderer() -> String {
    "OpenGL".to_string()
}

pub fn renderer_factory(renderer_name: &str) -> Result<Box<Renderer>, String> {
    match renderer_name {
        "OpenGL" => Ok(OpenGLRenderer::new()),
        _        => Err(format!("No renderer by the name {}", renderer_name)),
    }
}


