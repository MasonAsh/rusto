use common::*;
use super::transform::Transform;
use cgmath::perspective;

pub struct Camera {
    transform: Transform,
    projection: Mat4f,
}

impl Camera {
    pub fn new(fov: Degf, aspect: f32, near: f32, far: f32) -> Camera {
        Camera {
            transform: Transform::identity(),
            projection: perspective(fov, aspect, near, far)
        }
    }
    
    pub fn transform(&self) -> &Transform {
        &self.transform
    }
    
    pub fn transform_change(&mut self, fun: &Fn(&mut Transform)) {
        fun(&mut self.transform);
    }
    
    pub fn projection(&self) -> Mat4f {
        self.projection.clone()
    }
    
    pub fn view(&self) -> Mat4f {
        self.transform.to_matrix().invert().unwrap()
    }
}
