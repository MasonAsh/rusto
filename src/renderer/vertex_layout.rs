use std::mem::size_of;

pub enum VertexElementType {
    F32,
    F32F32,
    F32F32F32,
    F32F32F32F32,
}

impl VertexElementType {
    pub fn get_size(vtype: &VertexElementType) -> usize {
        match *vtype {
            VertexElementType::F32 => size_of::<f32>(),
            VertexElementType::F32F32 => size_of::<f32>() * 2,
            VertexElementType::F32F32F32 => size_of::<f32>() * 3,
            VertexElementType::F32F32F32F32 => size_of::<f32>() * 4,
        }
    }

    pub fn get_size_of(&self) -> usize {
        VertexElementType::get_size(self)
    }

    pub fn get_num_components(&self) -> i32 {
        match *self {
            VertexElementType::F32 => 1,
            VertexElementType::F32F32 => 2,
            VertexElementType::F32F32F32 => 3,
            VertexElementType::F32F32F32F32 => 4,
        }
    }
}

pub struct VertexElement {
    pub vtype: VertexElementType,
    pub name: String,
    pub offset: usize,
}

pub struct VertexLayoutDescription {
    pub elements: Vec<VertexElement>
}

impl VertexLayoutDescription {
    pub fn new() -> VertexLayoutDescription {
        VertexLayoutDescription {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, name: String, vtype: VertexElementType) {
        let offset: usize;

        match self.elements.last() {
            Some(x) => offset = x.offset + x.vtype.get_size_of(),
            None    => offset = 0,
        }

        self.elements.push(VertexElement {
            vtype: vtype,
            name: name,
            offset: offset,
        });
    }
}

