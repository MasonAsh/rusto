use std::path::Path;


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

pub struct MaterialPassData {
    blend_func: BlendFunc,
    vertex_inputs: Vec<Variable>,
    parameters: Vec<Variable>,
    code: String,    
}

pub struct MaterialFunctionData {
    code: String,
}

pub struct MaterialParseData {
    name: String,
    functions: MaterialFunctionData,
    vertex_attributes: VertexAttributeData,
    vertex_modifiers: VertexModifierData,
    passes: Vec<MaterialPassData>,
}

pub fn parse_material_file(path: &Path) -> MaterialParseData {
    panic!("Not implemented!")
}