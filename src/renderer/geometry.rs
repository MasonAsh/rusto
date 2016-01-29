use super::vertex_layout::VertexLayoutDescription;

pub trait Geometry {
    fn get_vertex_layout_description(&self) -> &VertexLayoutDescription;
}

