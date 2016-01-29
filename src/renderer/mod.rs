pub mod vertex_layout;
pub mod buffer;

pub use self::vertex_layout::*;
pub use self::buffer::*;

type Handle = usize;

pub type VBOHandle = Handle;
pub type VLayoutHandle = Handle;
pub type TextureHandle = Handle;
pub type IBOHandle = Handle;
pub type ProgramHandle = Handle;

pub enum IndexType {
    U16,
    U32,
}

pub trait Renderer {
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    fn create_vertex_layout(&mut self, desc: VertexLayoutDescription, vbo: VBOHandle) -> Result<VLayoutHandle, String>;
    fn create_vertex_buffer_object(&mut self, data: BufferData) -> Result<VBOHandle, String>;
    fn create_index_buffer_object(&mut self, index_type: IndexType, data: BufferData) -> Result<IBOHandle, String>;
    fn create_program(&mut self, vert_src: String, pix_src: String) -> Result<ProgramHandle, String>;
    fn draw(&mut self, vbos: VBOHandle, ibo: IBOHandle, program: ProgramHandle);
}

pub mod backends;

