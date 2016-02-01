extern crate cgmath;

use self::cgmath::*;

use std::mem;

pub enum ParamValue {
    F32(f32),
    Vec4(Vector4<f32>),
    Mat3(Matrix3<f32>),
    Mat4(Matrix4<f32>),
}

pub struct Param {
    pub name: String,
    pub value: ParamValue,
}

pub struct ParamGroup {
    pub name: String,
    pub params: Vec<Param>,
}

pub struct ShaderParams {
    groups: Vec<ParamGroup>,
    changes: Vec<String>, 
}

impl ShaderParams {
    pub fn new(groups: Vec<ParamGroup>) -> ShaderParams {
        return ShaderParams {
            groups: groups,
            changes: Vec::new(),
        }
    }
    
    fn find_mut_param(&mut self, name: &str) -> Option<&mut Param> {
        let mut result: Option<&mut Param> = None;
        for group in self.groups.iter_mut() {
            for param in group.params.iter_mut() {
                if param.name == name {
                    result = Some(&mut *param);
                    break;
                }
            }
        }
        
        result
    }

    fn find_param(&self, name: &str) -> Option<&Param> {
        let mut result: Option<&Param> = None;
        for group in self.groups.iter() {
            for param in group.params.iter() {
                if param.name == name {
                    result = Some(param);
                    break;
                }
            }
        }
        
        result
    }
    
    pub fn set(&mut self, name: &str, value: ParamValue) {
        {
            let param = self.find_mut_param(name).unwrap();
            param.value = value;
        }
        self.changes.push(name.clone().to_string());
    }

    pub fn get(&self, name: &str) -> &ParamValue {
        let param = self.find_param(name).unwrap();
        &param.value
    }
    
    pub fn flush_changes(&mut self) -> Vec<String> {
        mem::replace(&mut self.changes, Vec::new())
    }
}

