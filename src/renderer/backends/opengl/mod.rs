extern crate gl;

use super::super::*;

use std::mem;
use std::ptr;
use std::ffi::CString;
use std::str;

use self::gl::types::*;

type GLHandle = u32;

struct GLVbo {
    id: GLHandle,
    //layout: VLayoutHandle
}

struct GLIbo {
    id: GLHandle,
    itype: IndexType,
    count: usize,
}

struct GLProg {
    id: GLHandle,
    vsid: GLHandle,
    fsid: GLHandle,
}

struct GLVLayout {
    id: GLHandle,
    desc: VertexLayoutDescription,
    vbo: VBOHandle,
}

pub struct OpenGLRenderer {
    vlayouts: Vec<GLVLayout>,
    vbos: Vec<GLVbo>,
    ibos: Vec<GLIbo>,
    progs: Vec<GLProg>,
}

impl OpenGLRenderer {
    pub fn new() -> Box<OpenGLRenderer> {
        Box::new(OpenGLRenderer {
            vlayouts: Vec::new(),
            vbos: Vec::new(),
            ibos: Vec::new(),
            progs: Vec::new(),
        })
    }

    fn bind_vertex_buffer(&self, vboh: VBOHandle) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[vboh].id);
        }
    }

    fn bind_index_buffer(&self, iboh: IBOHandle) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibos[iboh].id);
        }
    }

    fn bind_program(&self, progh: ProgramHandle) {
        unsafe {
            gl::UseProgram(self.progs[progh].id);
        }
    }

    fn compile_shader(&self, src: String, shader_type: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            let c_str = CString::new(src).unwrap();

            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);


            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<&[u8]> = Vec::new();
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as * mut GLchar);
                panic!("{}", str::from_utf8(&buf[0]).ok().expect("ShaderInfoLog not valid utf8."));
            }

            shader
        }
    }
}

impl Renderer for OpenGLRenderer {
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn create_vertex_layout(&mut self, desc: vertex_layout::VertexLayoutDescription, vbo: VBOHandle) -> Result<VLayoutHandle, String> {
        let mut vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut vertex_size: i32 = 0;

            for elem in desc.elements.iter() {
                vertex_size += elem.vtype.get_size_of() as i32;
            }
            
            self.bind_vertex_buffer(vbo);

            for (i,elem) in desc.elements.iter().enumerate() {
                let index = i as u32;
                let num_components = elem.vtype.get_num_components();
                let elem_type = match elem.vtype {
                    VertexElementType::F32 | VertexElementType::F32F32 |
                    VertexElementType::F32F32F32 | VertexElementType::F32F32F32F32 => gl::FLOAT,
                };
                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(index, num_components, elem_type, gl::FALSE, vertex_size, mem::transmute(elem.offset));
            }
        }

        self.vlayouts.push(GLVLayout {
            id: vao,
            desc: desc,
            vbo: vbo,
        });

        Ok(self.vlayouts.len() - 1)
    }

    fn create_vertex_buffer_object(&mut self, data: BufferData) -> Result<VBOHandle, String> {
        let mut buf_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut buf_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buf_id);
            gl::BufferData(gl::ARRAY_BUFFER, data.bytes.len() as isize, mem::transmute(&data.bytes[0]), gl::STATIC_DRAW);
        }

        let vbo = GLVbo {
            id: buf_id,
            //layout: hlayout
        };

        self.vbos.push(vbo);

        Ok(self.vbos.len() - 1)
    }

    fn create_index_buffer_object(&mut self, itype: IndexType, data: BufferData) -> Result<IBOHandle, String> {
        let mut buf_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut buf_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buf_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, data.bytes.len() as isize, mem::transmute(&data.bytes[0]), gl::STATIC_DRAW);
        }

        let count;

        match itype {
            IndexType::U32 => count = data.bytes.len() / mem::size_of::<u32>(),
            IndexType::U16 => count = data.bytes.len() / mem::size_of::<u16>(),
        }

        let ibo = GLIbo {
            id: buf_id,
            itype: itype,
            count: count
        };

        self.ibos.push(ibo);

        Ok(self.vbos.len() - 1)
    }

    fn create_program(&mut self, vert_src: String, frag_src: String) -> Result<ProgramHandle, String> {
        let vs = self.compile_shader(vert_src, gl::VERTEX_SHADER);
        let fs = self.compile_shader(frag_src, gl::FRAGMENT_SHADER);

        let program = 0;

        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<&[u8]> = Vec::new();
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(&buf[0]).ok().expect("ProgramInfoLog not valid utf8"));
            }
        }

        let prog = GLProg {
            vsid: vs,
            fsid: fs,
            id: program
        };

        self.progs.push(prog);

        Ok(self.progs.len() - 1)
    }

    fn draw(&mut self, vboh: VBOHandle, iboh: IBOHandle, progh: ProgramHandle) {
        let vbo = &self.vbos[vboh];
        let ibo = &self.ibos[iboh];
        let prog = &self.progs[progh];

        self.bind_program(progh);
        self.bind_vertex_buffer(vboh);
        self.bind_index_buffer(iboh);

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, ibo.count as i32);
        }
    }
}

