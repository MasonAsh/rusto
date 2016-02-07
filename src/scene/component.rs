use common::*;
use super::transform::Transform;
use super::camera::Camera;
use renderer::Renderer;

pub trait SceneComponent {
    fn global_transform_change(&mut self, transform: &Transform);
    
    fn local_transform(&self) -> &Transform;
    fn local_transform_mut(&mut self) -> &mut Transform;
    
    fn render(&mut self, renderer: &mut Box<Renderer>, camera: &Camera);
}
