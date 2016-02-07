use common::*;
use std::ops::Add;

#[derive(Clone)]
pub struct Transform {
    pub position: Vec3f,
    pub rotation: Quatf,
    pub scale: Vec3f,
}

impl Transform {
    pub fn new(position: Vec3f, rotation: Quatf, scale: Vec3f) -> Transform {
        Transform {
            position: position,
            rotation: rotation,
            scale: scale,
        }
    }
    
    pub fn identity() -> Transform {
        Transform::new(Vec3f::zero(), Quatf::one(), Vec3f::new(1.0, 1.0, 1.0))
    }
    
    pub fn to_matrix(&self) -> Mat4f {
        Mat4f::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z) *
        Mat4f::from(self.rotation) *
        Mat4f::from_translation(self.position)
    }
}

impl Add for Transform {
    type Output = Transform;
    
    fn add(self, _rhs: Transform) -> Transform {
        Transform::new(
            self.position + _rhs.position,
            self.rotation * _rhs.rotation,
            self.scale * _rhs.scale
        )
    }
}
