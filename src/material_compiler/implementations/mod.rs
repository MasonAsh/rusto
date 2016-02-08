pub mod opengl_backend;
pub use self::opengl_backend::OpenGLDataGenerator;

use super::BackendDataGenerator;

/// Returns a list of all the BackendDataGenerator implementations.
/// Right now it's just OpenGL.
pub fn create_backend_data_generators() -> Vec<Box<BackendDataGenerator>> {
    vec![Box::new(OpenGLDataGenerator::new())]
}
