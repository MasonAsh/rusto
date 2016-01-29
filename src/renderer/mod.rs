pub mod vertex_layout;
pub mod buffer;
pub mod geometry;

pub use self::vertex_layout::*;
pub use self::buffer::*;
pub use self::geometry::*;


pub enum IndexType {
    U16,
    U32,
}

pub trait Renderer {
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

//    fn create_vertex_layout(&mut self, desc: VertexLayoutDescription, vbo: VBOHandle) -> Result<VLayoutHandle, String>;
//    fn create_vertex_buffer_object(&mut self, data: BufferData) -> Result<VBOHandle, String>;
//    fn create_index_buffer_object(&mut self, index_type: IndexType, data: BufferData) -> Result<IBOHandle, String>;
//    fn create_program(&mut self, vert_src: String, pix_src: String) -> Result<ProgramHandle, String>;
//    fn draw(&mut self, vbos: VBOHandle, ibo: IBOHandle, program: ProgramHandle);
	  fn create_geometry(&mut self, vertex_data: BufferData, index_data: BufferData, layout_desc: VertexLayoutDescription, index_type: IndexType, vert_src: &str, frag_src: &str) -> Box<Geometry>;

    fn draw_geometry(&mut self, geom: &Box<Geometry>);
}

pub mod backends;

