use std::path::Path;

use super::super::BackendDataGenerator;
use super::super::parser::*;

pub struct OpenGLDataGenerator;

impl OpenGLDataGenerator {
    pub fn new() -> OpenGLDataGenerator {
        OpenGLDataGenerator
    }
}

impl BackendDataGenerator for OpenGLDataGenerator {
    fn output_data(&mut self, parse: &MaterialParseData) -> Result<Vec<u8>, &str> {
        panic!("Not implemented")
    }
}