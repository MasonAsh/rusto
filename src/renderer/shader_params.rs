extern crate cgmath;

use self::cgmath::*;

use std::collections::HashMap;

pub enum ParamValue {
    Vec4(Vector4<f32>),
    Mat3(Matrix3<f32>),
    Mat4(Matrix4<f32>),
}

pub struct ShaderParams {
    params: HashMap<String, ParamValue>,
}

impl ShaderParams {
    fn new() -> ShaderParams {
        return ShaderParams {
            params: HashMap::new(),
        }
    }

    fn add_param(&mut self, name: &str, value: ParamValue) {
        self.params.insert(name.to_string(), value);
    }

//    fn get_param(&selfname: &str) -> ParamValue {
//        self.params.get(name)
//    }
}

