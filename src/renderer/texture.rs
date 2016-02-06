pub enum FilteringMethod {
    Nearest,
    BiLinear,
    TriLinear,
    Anisotropic,
}

pub enum TextureFormat {
    RGB,
    RGBA,
    Alpha,
    Luminance,
    LuminanceAlpha,
}

pub type TextureParamHandle = u32;

pub trait Texture {
    fn param_handle(&self) -> TextureParamHandle;
    fn format(&self) -> &TextureFormat;
    
    fn set_filtering_method(&mut self, method: FilteringMethod);
}
