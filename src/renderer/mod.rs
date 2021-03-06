pub mod vertex_layout;
pub mod buffer;
pub mod geometry;
pub mod texture;
pub mod shader_params;
pub mod util;

pub use self::vertex_layout::*;
pub use self::buffer::*;
pub use self::geometry::*;
pub use self::texture::*;
pub use self::shader_params::*;

use image::DynamicImage;

pub enum IndexType {
    U16,
    U32,
}

pub trait Renderer {
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    fn create_texture_from_image(&mut self, image_data: &DynamicImage) -> Box<Texture>;

    fn create_geometry(&mut self, vertex_data: &BufferData, index_data: &BufferData, layout_desc: &VertexLayoutDescription, index_type: IndexType, vert_src: &str, frag_src: &str) -> Box<Geometry>;
    fn draw_geometry(&mut self, geom: &mut Box<Geometry>);
}

pub mod backends;

