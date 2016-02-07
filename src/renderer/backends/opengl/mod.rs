extern crate gl;
extern crate cgmath;
extern crate image;

use super::super::*;

use std::mem;
use std::ptr;
use std::ffi::CString;
use std::str;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use self::gl::types::*;

use self::cgmath::*;

use self::image::{GenericImage, DynamicImage};

type GLHandle = u32;

type Handle = usize;

type VBOHandle = Handle;
type VAOHandle = Handle;
type IBOHandle = Handle;
type ProgramHandle = Handle;

struct GLVbo {
    id: GLHandle,
}

struct GLIbo {
    id: GLHandle,
    itype: IndexType,
    count: usize,
}

struct GLUniform {
    name: String,
    index: GLuint,
    offset: GLint,
    utype: GLenum,
    size: GLsizei,
}

struct GLUniformBlock {
    name: String,
    size: usize,
    buffer: GLuint,
    buffer_data: BufferData,
    uniforms: Vec<GLUniform>,
}

impl PartialEq for GLUniformBlock {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

struct GLSampler2D {
    uniform_info: GLUniform,
    tex_unit: u32,
    tex_id: GLuint,
}

struct GLProg {
    id: GLHandle,
    vsid: GLHandle,
    fsid: GLHandle,
    uniform_blocks: Vec<GLUniformBlock>,
    sampler2ds: Vec<GLSampler2D>,
}

struct GLVertexArrayObject {
    id: GLHandle,
}

struct GLStateManager {
    prog: GLHandle,
    vao: GLHandle,
    vbo: GLHandle,
    ibo: GLHandle,
    ubo: GLHandle,
    tex_units: HashMap<u32, GLuint>
}

impl GLStateManager {
    pub fn new() -> GLStateManager {
        GLStateManager {
            prog: 0,
            vao: 0,
            vbo: 0,
            ibo: 0,
            ubo: 0,
            tex_units: HashMap::new(),
        }
    }

    pub fn set_program(&mut self, prog: GLHandle) {
        if self.prog != prog {
            self.prog = prog;
            unsafe { gl::UseProgram(prog); }
        }
    }

    pub fn set_vao(&mut self, vao: GLHandle) {
        if self.vao != vao {
            self.vao = vao;
            unsafe { gl::BindVertexArray(vao); }
        }
    }

    pub fn set_vbo(&mut self, vbo: GLHandle) {
        if self.vbo != vbo {
            self.vbo = vbo;
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo); }
        }
    }

    pub fn set_ibo(&mut self, ibo: GLHandle) {
        if self.ibo != ibo {
            self.ibo = ibo;
            unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo); }
        }
    }

    pub fn set_ubo(&mut self, ubo: GLHandle) {
        if self.ubo != ubo {
            self.ubo = ubo;
            unsafe { gl::BindBuffer(gl::UNIFORM_BUFFER, ubo); }
        }
    }

    pub fn set_tex2d(&mut self, tex_unit_i: u32, tex2d: GLuint) {
        unsafe {
            let mut bind = false;
            
            match self.tex_units.entry(tex_unit_i) {
                Vacant(entry) => { 
                    entry.insert(tex2d);
                    bind = true;
                },
                Occupied(mut entry) => {
                    let oldvalue = entry.insert(tex2d);
                    if oldvalue != tex2d {
                        bind = true;
                    }
                },
            }
            
            if bind {
                gl::ActiveTexture(gl::TEXTURE0 + tex_unit_i);
                gl::BindTexture(gl::TEXTURE_2D, tex2d);
            }
        }
    }
}

pub struct OpenGLTexture {
    id: GLHandle,
    filter_method: FilteringMethod,
    filter_changed: bool,
    texture_format: TextureFormat,
}

impl Texture for OpenGLTexture {
    fn param_handle(&self) -> TextureParamHandle {
        self.id
    }
    
    fn format(&self) -> &TextureFormat {
        &self.texture_format
    }
    
    fn set_filtering_method(&mut self, method: FilteringMethod) {
        self.filter_method = method;
        self.filter_changed = true;
    }
}

pub struct OpenGLGeometry {
    vbo: VBOHandle,
    ibo: IBOHandle,
    vao: VAOHandle,
    program: ProgramHandle,
    layout_desc: VertexLayoutDescription,
    params: ShaderParams,
}

impl Geometry for OpenGLGeometry {
    fn get_vertex_layout_description(&self) -> &VertexLayoutDescription {
        &self.layout_desc
    }
    
    fn get_params(&self) -> &ShaderParams {
        &self.params
    }
    
    fn get_mut_params(&mut self) -> &mut ShaderParams {
        &mut self.params
    }
}

pub struct OpenGLRenderer {
    vaos: Vec<GLVertexArrayObject>,
    vbos: Vec<GLVbo>,
    ibos: Vec<GLIbo>,
    progs: Vec<GLProg>,
    state: GLStateManager,
}

impl OpenGLRenderer {
    pub fn new() -> Box<OpenGLRenderer> {
        Box::new(OpenGLRenderer {
            vaos: Vec::new(),
            vbos: Vec::new(),
            ibos: Vec::new(),
            progs: Vec::new(),
            state: GLStateManager::new(),
        })
    }

    fn compile_shader(&self, src: &str, shader_type: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            let c_str = CString::new(src.as_bytes()).unwrap();

            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);


            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as * mut GLchar);
                panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8."));
            }

            shader
        }
    }

    fn create_vertex_array_object(&mut self, desc: &VertexLayoutDescription, vboh: VBOHandle, progh: ProgramHandle) -> Result<VAOHandle, String> {
        let mut vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            self.state.set_vao(vao);

            let vboid = self.vbos[vboh].id;
            self.state.set_vbo(vboid);

            let progid = self.progs[progh].id;

            self.state.set_program(progid);

			let mut stride: GLsizei = 0;
			for elem in desc.elements.iter() {
				stride += elem.vtype.get_size_of() as i32;
			}

            for (i,elem) in desc.elements.iter().enumerate() {
                let index = i as u32;
                let num_components = elem.vtype.get_num_components();
                let elem_type = match elem.vtype {
                    VertexElementType::F32 | VertexElementType::F32F32 |
                    VertexElementType::F32F32F32 | VertexElementType::F32F32F32F32 => gl::FLOAT,
                };
                
                let attr_name_cstr = CString::new(elem.name.clone()).unwrap().as_ptr();
                
                gl::BindAttribLocation(progid, index, attr_name_cstr);
                
                gl::EnableVertexAttribArray(index as u32);
                gl::VertexAttribPointer(index as u32, num_components, elem_type, gl::FALSE, stride, mem::transmute(elem.offset));
            }
        }

        self.vaos.push(GLVertexArrayObject {
            id: vao,
        });

        Ok(self.vaos.len() - 1)
    }

    fn create_vertex_buffer_object(&mut self, data: &BufferData) -> Result<VBOHandle, String> {
        let mut buf_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut buf_id);
            self.state.set_vbo(buf_id);
            gl::BufferData(gl::ARRAY_BUFFER, data.bytes.len() as isize, mem::transmute(&data.bytes[0]), gl::STATIC_DRAW);
        }

        let vbo = GLVbo {
            id: buf_id,
        };

        self.vbos.push(vbo);

        Ok(self.vbos.len() - 1)
    }

    fn create_index_buffer_object(&mut self, itype: IndexType, data: &BufferData) -> Result<IBOHandle, String> {
        let mut buf_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut buf_id);
            self.state.set_ibo(buf_id);
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

    fn create_program(&mut self, vert_src: &str, frag_src: &str) -> Result<ProgramHandle, String> {
        let vs = self.compile_shader(vert_src, gl::VERTEX_SHADER);
        let fs = self.compile_shader(frag_src, gl::FRAGMENT_SHADER);

        let program;

        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(&buf).ok().expect("programinfolog not valid utf8"));
            }
        }

        let uniform_blocks = self.get_program_uniform_blocks(program);
        let sampler2ds = self.get_program_samplers(program);

        let prog = GLProg {
            vsid: vs,
            fsid: fs,
            id: program,
            uniform_blocks: uniform_blocks,
            sampler2ds: sampler2ds,
        };

        self.progs.push(prog);

        Ok(self.progs.len() - 1)
    }
    
    fn get_shader_params(&self, uniform_blocks: &Vec<GLUniformBlock>, samplers: &Vec<GLSampler2D>) -> ShaderParams {
        let mut param_groups: Vec<ParamGroup> = Vec::with_capacity(uniform_blocks.len());

        for block in uniform_blocks.iter() {
            let group_name = block.name.clone();
            let mut params: Vec<Param> = Vec::with_capacity(block.uniforms.len());
            
            for uniform in block.uniforms.iter() {
                let param_value: ParamValue = match uniform.utype {
                    gl::FLOAT      => ParamValue::F32(0.0),
                    gl::FLOAT_VEC4 => ParamValue::Vec4(vec4(0.0, 0.0, 0.0, 0.0)),
                    gl::FLOAT_MAT3 => ParamValue::Mat3(Matrix3::identity()),
                    gl::FLOAT_MAT4 => ParamValue::Mat4(Matrix4::identity()),
                    _              => panic!("Unsupported shader uniform type!"),
                };

                params.push(Param {
                    name: uniform.name.clone(),
                    value: param_value
                });
            }

            param_groups.push(ParamGroup {
                name: group_name,
                params: params,
            });
        }
        
        let tex_group_name = "_textures".to_string();
        let mut tex_params: Vec<Param> = Vec::with_capacity(samplers.len());
        
        for sampler in samplers.iter() {
            tex_params.push(Param {
                name: sampler.uniform_info.name.clone(),
                value: ParamValue::Texture2D(0),
            });
        }
        
        param_groups.push(ParamGroup {
            name: tex_group_name,
            params: tex_params, 
        });

        ShaderParams::new(param_groups)
    }
    
    fn get_uniform_info(&self, progid: GLuint, uniform_index: u32) -> GLUniform {
        unsafe {
            let mut max_uniform_name_len: GLint = 0;
            gl::GetProgramiv(progid, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut max_uniform_name_len);
            
            let mut name_len: GLsizei = 0;
            let mut uniform_name_bytes = Vec::with_capacity(max_uniform_name_len as usize);
            uniform_name_bytes.set_len(max_uniform_name_len as usize);
            let mut uniform_type: GLenum = 0;
            let mut uniform_size: GLint = 0;
            
            gl::GetActiveUniform(progid,
                                uniform_index as u32,
                                max_uniform_name_len,
                                &mut name_len,
                                &mut uniform_size,
                                &mut uniform_type,
                                uniform_name_bytes.as_mut_ptr() as *mut GLchar);
            
            for _ in 0..(max_uniform_name_len - name_len) {
                uniform_name_bytes.pop();
            }

            let uniform_name: String = str::from_utf8(&uniform_name_bytes).unwrap().to_string();
            
            let mut uniform_offset: GLint = 0;
            gl::GetActiveUniformsiv(progid, 1, &uniform_index, gl::UNIFORM_OFFSET, &mut uniform_offset);
            
            GLUniform {
                name: uniform_name,
                index: uniform_index,
                offset: uniform_offset,
                utype: uniform_type,
                size: uniform_size,
            }
        }
    }
    
    fn get_program_uniform_blocks(&mut self, progid: GLuint) -> Vec<GLUniformBlock> {
        let mut num_blocks: GLint = 0;
        unsafe { gl::GetProgramiv(progid, gl::ACTIVE_UNIFORM_BLOCKS, &mut num_blocks); }
        
        let mut max_uniform_name_len: GLint = 0;
        unsafe { gl::GetProgramiv(progid, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut max_uniform_name_len); }
        
        let mut uniform_blocks: Vec<GLUniformBlock> = Vec::with_capacity(num_blocks as usize);
        
        unsafe {
            for i in 0..num_blocks {
                let mut name_len: GLint = 0;
                
                gl::GetActiveUniformBlockiv(progid, i as u32, gl::UNIFORM_BLOCK_NAME_LENGTH, &mut name_len);
                
                let mut name_bytes = Vec::with_capacity(name_len as usize);
                name_bytes.set_len((name_len as usize) - 1);

                gl::GetActiveUniformBlockName(progid, i as u32, name_len, ptr::null_mut(), name_bytes.as_mut_ptr() as *mut GLchar);
                let block_name: String = str::from_utf8(&name_bytes).unwrap().to_string();
    
                let mut block_size: GLint = 0;
                gl::GetActiveUniformBlockiv(progid, i as u32, gl::UNIFORM_BLOCK_DATA_SIZE, &mut block_size);

                let mut num_uniforms: GLint = 0;
                gl::GetActiveUniformBlockiv(progid, i as u32, gl::UNIFORM_BLOCK_ACTIVE_UNIFORMS, &mut num_uniforms);
    
                let mut uniform_indices = Vec::with_capacity(num_uniforms as usize);
                
                uniform_indices.set_len(num_uniforms as usize);
                
                gl::GetActiveUniformBlockiv(progid,
                                            i as u32,
                                            gl::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES,
                                            uniform_indices.as_mut_ptr() as *mut GLint);
    
                let mut uniforms: Vec<GLUniform> = Vec::with_capacity(num_uniforms as usize);
                
                for uniform_index in uniform_indices {
                    let uniform = self.get_uniform_info(progid, uniform_index);
                    
                    uniforms.push(uniform);
                }

                let buffer_data = BufferData::new_zero_initialized(block_size as usize);

                let mut ubo: GLuint = 0;
                gl::GenBuffers(1, &mut ubo);
                self.state.set_ubo(ubo);
                gl::BufferData(gl::UNIFORM_BUFFER, buffer_data.bytes.len() as isize, mem::transmute(&buffer_data.bytes[0]), gl::DYNAMIC_DRAW);
                
                gl::UniformBlockBinding(progid, i as u32, i as u32);
                gl::BindBufferBase(gl::UNIFORM_BUFFER, i as u32, ubo);
                
                uniform_blocks.push(GLUniformBlock {
                    name: block_name,
                    size: block_size as usize,
                    buffer: ubo,
                    buffer_data: BufferData::new_zero_initialized(block_size as usize),
                    uniforms: uniforms,
                });
            }

            uniform_blocks
        }
    }
    
    fn get_program_samplers(&self, progid: GLHandle) -> Vec<GLSampler2D> {
        let mut num_uniforms: GLint = 0;
        
        unsafe {
            gl::GetProgramiv(progid, gl::ACTIVE_UNIFORMS, &mut num_uniforms);
        }
        
        let mut samplers = Vec::new();
        
        for i in 0..num_uniforms {
            let mut utype: GLint = 0;
            
            unsafe {
                gl::GetActiveUniformsiv(progid, 1, &(i as u32) as *const u32, gl::UNIFORM_TYPE, &mut utype);
            }
            
            match utype as u32 {
                gl::SAMPLER_2D => {
                    let uniform_info = self.get_uniform_info(progid, i as u32);
                    let tex_unit = samplers.len() as u32;
                    
                    samplers.push(GLSampler2D {
                       uniform_info: uniform_info,
                       tex_unit: tex_unit,
                       tex_id: 0, 
                    });
                },
                _ => continue,
            }
        }
        
        samplers
    }

    fn draw_vertex_arrays(&mut self, vboh: VBOHandle, vaoh: VAOHandle, iboh: IBOHandle, progh: ProgramHandle) {
        let ibo = &self.ibos[iboh];
        let prog = &self.progs[progh];

        self.state.set_program(self.progs[progh].id);
        self.state.set_vbo(self.vbos[vboh].id);
        self.state.set_ibo(self.ibos[iboh].id);
        self.state.set_vao(self.vaos[vaoh].id);
        
        for sampler in prog.sampler2ds.iter() {
            self.state.set_tex2d(sampler.tex_unit, sampler.tex_id);
        }

        let gl_itype = match ibo.itype {
            IndexType::U16 => gl::UNSIGNED_SHORT,
            IndexType::U32 => gl::UNSIGNED_INT,
        };

        unsafe {
            gl::DrawElements(gl::TRIANGLES, ibo.count as i32, gl_itype, ptr::null());
        }
    }

    fn apply_shader_params(&mut self, geom: &mut Box<OpenGLGeometry>) {
        let changes;

        {
            let mut params = geom.get_mut_params();
            changes = params.flush_changes();
        }

        let mut prog: &mut GLProg = self.progs.get_mut(geom.program).unwrap();
        let mut uniform_blocks: &mut Vec<GLUniformBlock> = &mut prog.uniform_blocks;
        let mut affected_blocks: Vec<usize> = Vec::new();

		self.state.set_program(prog.id);

        let params = geom.get_mut_params();

        // This is O(scary)
        // should probably be optimized some time
        // ShaderParams::flush_changes should return which blocks
        // are affected as well as the parameters within that where
        // affected, so that we can avoid this
        for name in changes.iter() {
            match *params.get(name) {
                ParamValue::Texture2D(tex_handle) => {
                    for sampler in prog.sampler2ds.iter_mut() {
                        if sampler.uniform_info.name == *name {
                            sampler.tex_id = tex_handle;
                            unsafe { gl::Uniform1i(sampler.uniform_info.index as i32, sampler.tex_unit as i32); }
                            break;
                        }
                    }
                },
                _ => (),
            }
            
            'outer: for (block_idx, block) in uniform_blocks.iter_mut().enumerate() {
                for uniform in block.uniforms.iter() {
                    if uniform.name == *name {
                        if !affected_blocks.contains(&block_idx) {
                            affected_blocks.push(block_idx);
                        }

                        let param_value = params.get(name);
                        match *param_value {
                            ParamValue::F32(x)  => block.buffer_data.update_region(uniform.offset as usize, vec![x]),
                            ParamValue::Vec4(x) => block.buffer_data.update_region(uniform.offset as usize, vec![x]),
                            ParamValue::Mat3(x) => block.buffer_data.update_region(uniform.offset as usize, vec![x]),
                            ParamValue::Mat4(x) => block.buffer_data.update_region(uniform.offset as usize, vec![x]),
                            ParamValue::Texture2D(x) => (),
                        }

                        break 'outer;
                    }
                }
            }
        }

        for block_idx in affected_blocks {
            let block = uniform_blocks.get_mut(block_idx).unwrap();
            unsafe {
                self.state.set_ubo(block.buffer);
                gl::BufferSubData(gl::UNIFORM_BUFFER, 0, block.buffer_data.bytes.len() as isize, mem::transmute(&block.buffer_data.bytes[0]));
            }
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
    
    fn create_texture_from_image(&mut self, image_data: &DynamicImage) -> Box<Texture> {
        unsafe {
            let mut tex_id: GLHandle = 0;
            gl::GenTextures(1, &mut tex_id);
            
            self.state.set_tex2d(0, tex_id);
            
            let tex_format = match *image_data {
                DynamicImage::ImageLuma8(_) => TextureFormat::Luminance,
                DynamicImage::ImageLumaA8(_) => TextureFormat::LuminanceAlpha,
                DynamicImage::ImageRgb8(_) => TextureFormat::RGB,
                DynamicImage::ImageRgba8(_) => TextureFormat::RGBA,
            };
            
            let gl_tex_format = match tex_format {
                TextureFormat::Luminance => gl::RED,
                TextureFormat::LuminanceAlpha => gl::RG,
                TextureFormat::RGB => gl::RGB,
                TextureFormat::RGBA => gl::RGBA,
                TextureFormat::Alpha => gl::ALPHA,
            };
            
            let (width, height) = image_data.dimensions();
            
            let mut raw_pixels = image_data.raw_pixels();
            
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl_tex_format as i32, width as i32, height as i32, 0, gl_tex_format, gl::UNSIGNED_BYTE, raw_pixels.as_mut_ptr() as *mut GLvoid);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            
            Box::new(OpenGLTexture {
               id: tex_id,
               filter_method: FilteringMethod::Nearest,
               filter_changed: false,
               texture_format: tex_format,
            })
        }
    }

    fn create_geometry(&mut self, vertex_data: &BufferData, index_data: &BufferData, layout: &VertexLayoutDescription, index_type: IndexType, vert_src: &str, frag_src: &str) -> Box<Geometry> {
        let vbo = self.create_vertex_buffer_object(vertex_data).unwrap();
        let prog = self.create_program(vert_src, frag_src).unwrap();
        let vao = self.create_vertex_array_object(&layout, vbo, prog).unwrap();
        let ibo = self.create_index_buffer_object(index_type, index_data).unwrap();
        
        let uniform_blocks = &self.progs[prog].uniform_blocks;
        let samplers = &self.progs[prog].sampler2ds;
        let params = self.get_shader_params(uniform_blocks, samplers);

        let geom = OpenGLGeometry {
            vbo: vbo,
            vao: vao,
            ibo: ibo,
            program: prog,
            layout_desc: layout.clone(),
            params: params
        };

        Box::new(geom)
    }

    fn draw_geometry(&mut self, geom: &mut Box<Geometry>) {
        // This is pretty lame. There should be a better way to convert Box<Geometry> to Box<OpenGLGeometry>
        // Perhaps this is just an unsafe design by nature however.
        let glgeom: &mut Box<OpenGLGeometry> = unsafe { mem::transmute(geom) };

        self.apply_shader_params(glgeom);

        self.draw_vertex_arrays(glgeom.vbo, glgeom.vao, glgeom.ibo, glgeom.program);
    }
}
