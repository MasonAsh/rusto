extern crate glium;

#[derive(Copy, Clone)]
pub struct VPosition {
    position: (f32, f32, f32),
}

implement_vertex!(VPosition, position);

#[derive(Copy, Clone)]
pub struct VNormal {
    normal: (f32, f32, f32),
}

implement_vertex!(VNormal, normal);

#[derive(Copy, Clone)]
pub struct VTexCoord {
    tex_coord: (f32, f32)
}

implement_vertex!(VTexCoord, tex_coord);

#[derive(Copy, Clone)]
pub struct VColor {
    color: (f32, f32, f32),
}

implement_vertex!(VColor, color);

pub struct Geometry {
    vertex_buffers: Vec<glium::vertex::VertexBufferAny>,
    index_buffer: glium::IndexBuffer<u32>,
}

impl Geometry {
    pub fn new(vertex_buffers: Vec<glium::vertex::VertexBufferAny>,
               index_buffer: glium::IndexBuffer<u32>)
               -> Geometry {
        Geometry {
            vertex_buffers: vertex_buffers,
            index_buffer: index_buffer,
        }
    }
}

pub struct GeometryBuilder {
    positions: Option<Vec<VPosition>>,
    normals: Option<Vec<VNormal>>,
    tex_coords: Option<Vec<VTexCoord>>,
    colors: Option<Vec<VColor>>,
    indices: Vec<u32>,
}

impl GeometryBuilder {
    pub fn new() -> GeometryBuilder {
        GeometryBuilder {
            positions: None,
            normals: None,
            tex_coords: None,
            colors: None,
            indices: Vec::new(),
        }
    }

    pub fn positions(mut self, positions: Vec<VPosition>) -> GeometryBuilder {
        self.positions = Some(positions);
        self
    }

    pub fn normals(mut self, normals: Vec<VNormal>) -> GeometryBuilder {
        self.normals = Some(normals);
        self
    }

    pub fn tex_coords(mut self, tex_coords: Vec<VTexCoord>) -> GeometryBuilder {
        self.tex_coords = Some(tex_coords);
        self
    }

    pub fn colors(mut self, colors: Vec<VColor>) -> GeometryBuilder {
        self.colors = Some(colors);
        self
    }

    pub fn indices(mut self, indices: Vec<u32>) -> GeometryBuilder {
        self.indices = indices;
        self
    }

    pub fn build(self, display: &glium::Display) -> Geometry {
        let mut vertex_buffers: Vec<glium::vertex::VertexBufferAny> = Vec::new();
        let mut index_buffer: glium::IndexBuffer<u32> = glium::IndexBuffer::new(display,
                                                                                glium::index::PrimitiveType::TrianglesList,
                                                                                &self.indices
                                                                                ).unwrap();

        match self.positions {
            Some(x) => vertex_buffers.push(
                glium::VertexBuffer::new(display, &x).unwrap().into()
                ),
            None    => ()
        }

        match self.normals {
            Some(x) => vertex_buffers.push(
                glium::VertexBuffer::new(display, &x).unwrap().into()
                ),
            None    => ()
        }

        match self.tex_coords {
            Some(x) => vertex_buffers.push(
                glium::VertexBuffer::new(display, &x).unwrap().into()
                ),
            None    => ()
        }

        match self.colors {
            Some(x) => vertex_buffers.push(
                glium::VertexBuffer::new(display, &x).unwrap().into()
                ),
            None    => ()
        }

        Geometry {
            vertex_buffers: vertex_buffers,
            index_buffer: index_buffer,
        }
    }
}
