extern crate rusto;
extern crate xml;

use rusto::material_compiler::parser::parse_material_file;
use rusto::material_compiler::MaterialCompiler;

use std::env;
use std::path::Path;

fn main() {
    // FIXME: use argparse.
    let args: Vec<String> = env::args().collect();
    let ref input = args[1];
    let ref output = args[2];
    
    let compiler = MaterialCompiler::new();
    
    let parse = parse_material_file(&Path::new(&input)).unwrap();
    
    compiler.compile(parse, &Path::new(&output));
}