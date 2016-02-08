mod implementations;
pub mod parser;

use self::implementations::create_backend_data_generators;
use self::parser::*;

use std::error::Error;
use std::path::Path;

/// Used to output information into compiled materials specific
/// to a renderer backend.
///
/// The OpenGL backend, for example, would output glsl shader code.
pub trait BackendDataGenerator {
    fn output_data(&mut self, ast: &MaterialParseData) -> Result<Vec<u8>, &str>;
}

pub struct MaterialCompiler {
    backends: Vec<Box<BackendDataGenerator>>
}

impl MaterialCompiler {
    pub fn new() -> MaterialCompiler {
        MaterialCompiler {
            backends: create_backend_data_generators(),
        }
    }
    
    pub fn compile(&self, parse_data: MaterialParseData, output_path: &Path) -> Option<String> {
        panic!("Not implemented");
    }
}
