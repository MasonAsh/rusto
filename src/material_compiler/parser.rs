use std::path::Path;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use std::fmt;
use std::str;
use std::io::Bytes;

use xml::{Event, Parser, Element, ElementBuilder};

/// Creates an enum with from_str and to_str methods.
/// Example:
/// enum_with_string_conversion!(MyEnum,
///     (EnumValue1, "value1"),
/// );
/// MyEnum::from_str("value1") produces Some(MyEnum::EnumValue1)
/// MyEnum::EnumValue1.to_str() produces "value1"
macro_rules! enum_with_string_conversion {
    (
        $enum_ident:ident,
        $(($variant_name:ident, $as_string:expr)),*,
    ) => (
        pub enum $enum_ident {
            $(
                $variant_name,  
            )*
        }
        
        impl $enum_ident {
            pub fn to_str(&self) -> &str {
                match *self {
                $(
                    $enum_ident::$variant_name => $as_string,    
                )*
                }
            }
            
            pub fn from_str(s: &str) -> Option<$enum_ident> {
                match s {
                $(
                    $as_string => Some($enum_ident::$variant_name), 
                )*
                    _ => None,
                }
            }
        }
    );
}

enum_with_string_conversion!(
    BlendFunc,
    (Add, "Add"),
    (Subtract, "Subtract"),
    (Multiply, "Multiply"),
);

enum_with_string_conversion!(
    VariableType,
    (Float, "float"),
    (Vec2, "vec2"),
    (Vec3, "vec3"),
    (Vec4, "vec4"),
    (Mat3, "mat3"),
    (Mat4, "mat4"),
);

pub struct Variable {
    name: String,
    vtype: VariableType,
}

pub struct VertexAttributeData {
    attribs: Vec<String>,
}

pub struct VertexModifierData {
    outputs: Vec<Variable>,
    code: String,
}

pub struct Param {
    vtype: VariableType,
    name: String,
    editor_type: Option<String>,
}

pub struct MaterialPassData {
    blend_func: BlendFunc,
    parameters: Vec<Param>,
    code: String,    
}

pub struct MaterialParseData {
    name: Option<String>,
    functions: Option<String>,
    vertex_attributes: Option<Vec<Variable>>,
    vertex_modifier: Option<VertexModifierData>,
    passes: Option<Vec<MaterialPassData>>,
}

impl MaterialParseData {
    fn new() -> MaterialParseData {
        MaterialParseData {
            name: None,
            functions: None,
            vertex_attributes: None,
            vertex_modifier: None,
            passes: None,
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    description: String,
}

impl ParseError {
    fn new(description: &str) -> ParseError {
        ParseError {
            description: description.to_string(),
        }
    }
}

fn variable_tag(element: &Element) -> Option<Variable> {
    let mut vartype: Option<VariableType> = None;
    let mut name: Option<String> = None;
    
    match element.get_attribute("type", None) {
        Some(x) => {
            match VariableType::from_str(x) {
                Some(x) => vartype = Some(x),
                None    => {}
            }
        },
        None => {},
    }
    
    match element.get_attribute("name", None) {
        Some(x) => {
            name = Some(x.to_string());
        },
        None => {}
    }
    
    if vartype.is_none() || name.is_none() {
        None
    } else {
        Some(Variable{
            vtype: vartype.unwrap(),
            name: name.unwrap(),
        })
    }
}

fn param_tag(element: &Element) -> Option<Param> {
    let mut vtype: Option<VariableType> = None;
    let mut name: Option<String> = None;
    let mut editor_type: Option<String> = None;
    
    match element.get_attribute("type", None) {
        Some(x) => {
            match VariableType::from_str(x) {
                Some(x) => vtype = Some(x),
                None    => {}
            }
        },
        None => {},
    }
    
    match element.get_attribute("name", None) {
        Some(x) => {
            name = Some(x.to_string());
        },
        None => {}
    }
    
    match element.get_attribute("editor_type", None) {
        Some(x) => {
            editor_type = Some(x.to_string());
        },
        None => {}
    }
    
    if vtype.is_none() || name.is_none() || editor_type.is_none() {
        None
    } else {
        Some(Param {
            vtype: vtype.unwrap(),
            name: name.unwrap(),
            editor_type: editor_type,
        })
    }
}

fn get_vars(element: &Element) -> Vec<Variable> {
    let variables = element.get_children("Variable", None);
                        
    let variables = variables
        .map(|x| variable_tag(x))
        .filter_map(|x| x)
        .collect();
 
    variables
}

fn do_material(element: &Element, parse_data: &mut MaterialParseData) {
    let mut passes = Vec::new();
    
    match element.get_child("Functions", None) {
        Some(e) => {
            parse_data.functions = Some(e.content_str());
        },
        None => {},
    }
    
    match element.get_child("VertexAttributes", None) {
        Some(e) => {
            let vars = get_vars(&e);
            parse_data.vertex_attributes = Some(vars);
        },
        None => {},
    }
    
    match element.get_child("VertexModifier", None) {
        Some(e) => {
            let mut outputs = Vec::new();
            let mut code = String::new();
            
            match e.get_child("Outputs", None) {
                Some(x) => {
                    outputs = get_vars(x);
                },
                None => {}
            }
            
            match e.get_child("Code", None) {
                Some(x) => {
                    code = x.content_str();
                },
                None => {}
            }
            
            parse_data.vertex_modifier = Some(VertexModifierData {
                outputs: outputs,
                code: code,
            });
        },
        None => {},
    }
    
    match element.get_child("Pass", None) {
        Some(e) => {
            let mut blend_func: Option<BlendFunc> = None;
            let mut params: Vec<Param> = Vec::new();
            let mut code: String = String::new();
            
            match e.get_attribute("BlendFunc", None) {
                Some(x) => {
                    blend_func = Some(BlendFunc::from_str(x).unwrap());
                },
                None => {},
            }
            
            match e.get_child("Parameters", None) {
                Some(x) => {
                    let param_elems = x.get_children("Param", None);
                    
                    params = param_elems
                        .map(|x| param_tag(x))
                        .filter_map(|x| x)
                        .collect();
                },
                None => {}
            }
            
            match e.get_child("Code", None) {
                Some(x) => {
                    code = x.content_str().to_string();
                },
                None => {},
            }
            
            passes.push(MaterialPassData {
                blend_func: blend_func.unwrap_or(BlendFunc::Add),
                parameters: params,
                code: code,
            });
        },
        None => {},
    }
}

pub fn parse_material_file(path: &Path) -> Result<MaterialParseData, ParseError> {
    let mut file = File::open(path).unwrap();
    
    let mut parse_data = MaterialParseData::new();
    
    let mut file_content = String::new();
    
    match file.read_to_string(&mut file_content) {
        Ok(_) => {},
        Err(x) => { return Err(ParseError::new(x.description())); },
    }
    
    let mut parser = Parser::new();
    let mut e = ElementBuilder::new();
    let mut passes = Vec::new();
    
    parser.feed_str(file_content.as_ref());
    
    let mut found_mat = false;
    
    for elem in parser.filter_map(|x| e.handle_event(x)) {
        match elem {
            Ok(e) => {
                match e.name.as_ref() {
                    "Material" => {
                        if found_mat {
                            return Err(ParseError::new("Only one material allowed per file."));
                        }
                        
                        match e.get_attribute("name", None) {
                            Some(x) => { parse_data.name = Some(x.to_string()); },
                            None => {},
                        }
                        
                        do_material(&e, &mut parse_data);
                        
                        found_mat = true;
                    }
                    _ => {
                        return Err(ParseError::new(format!("Unrecognized element {}", e.name).as_ref()));
                    }
                }
            },
            Err(e) => { return Err(ParseError::new(e.description())) },
        }
    }
    
    parse_data.passes = Some(passes);
    
    Ok(parse_data)
}
