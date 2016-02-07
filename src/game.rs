use std::thread;
use std::fs::File;
use std::path::Path;
use config::load_config_file;
use sdl2;
use sdl2::event::{Event};
use gl;
use image::GenericImage;
use cgmath::Vector3;
use cgmath::{Matrix4, Matrix};
use cgmath::perspective;
use cgmath::deg;
use cgmath::Point3;

use renderer::*;
use renderer::backends::{renderer_factory, determine_best_renderer};
use renderer::util::mesh::{MeshOptions, load_meshes_from_file};

pub struct Game {
    running: bool,
    sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    events: sdl2::EventPump,
    vid_ctx: sdl2::VideoSubsystem,
    gl_ctx: sdl2::video::GLContext,
    renderer: Box<Renderer>,
    geometry: Box<Geometry>,
}

impl Game {
    pub fn new() -> Game {
        let sdl = sdl2::init().unwrap();
        let vid_ctx = sdl.video().unwrap();

        let config = load_config_file();

        let width: u32;
        let window_title: String;
        let height: u32;
        let fullscreen: bool;

        match config.get("width") {
            Some(x) => width = x.parse::<u32>().unwrap_or(640),
            None    => width = 640,
        }

        match config.get("height") {
            Some(x) => height = x.parse::<u32>().unwrap_or(480),
            None    => height = 480,
        }

        match config.get("window_title") {
            Some(x) => window_title = x.clone(),
            None    => window_title = "No title?".to_string(),
        }

        match config.get("fullscreen") {
            Some(x) => match x.as_ref() {
                "true"  => fullscreen = true,
                "false" => fullscreen = false,
                _       => {
                    println!("Invalid fullscreen value in config. Defaulting to false.");
                    fullscreen = false;
                }
            },
            None    => {
                fullscreen = false;
            }
        }

        let mut window: sdl2::video::Window;

        if !fullscreen {
            window = vid_ctx.window(window_title.as_ref(), width, height).position_centered().opengl().build().unwrap();
        } else {
            window = vid_ctx.window(window_title.as_ref(), width, height).position_centered().opengl().fullscreen().build().unwrap();
        }

        window.show();
        let events = sdl.event_pump().unwrap();

        let gl_ctx = window.gl_create_context().unwrap();

        match window.gl_make_current(&gl_ctx) {
            Ok(_)  => (),
            Err(x) => panic!("failed to bind window to OpenGL. Reason: {}", x)
        }

        gl::load_with(|name| vid_ctx.gl_get_proc_address(name) as *const _);

        let renderer_name = determine_best_renderer();
        let mut renderer = renderer_factory(&renderer_name).unwrap();

        // let mut vdesc = VertexLayoutDescription::new();
        // vdesc.add_element("position".to_string(), VertexElementType::F32F32);
        // vdesc.add_element("tex_coord".to_string(), VertexElementType::F32F32);

        // let tri: Vec<f32> = vec![
        //     -0.5f32, -0.5,
        //     0.0, 0.0,
        //     0.0, 0.5,
        //     0.5, 1.0,
        //     0.5, -0.5,
        //     1.0, 0.0,  
        // ];

        // let vertex_data = BufferData::new_initialized(tri);

        // let index_vec: Vec<u32> = vec![
        //     1, 0, 2
        // ];

        // let index_data = BufferData::new_initialized(index_vec);

        let mesh_data = load_meshes_from_file(&Path::new("data/sphere.obj"), &MeshOptions::default()).unwrap();
        
        let vdesc = &mesh_data[0].layout;
        let vertex_data = &mesh_data[0].vertex_data;
        let index_data = &mesh_data[0].index_data;

        let vert_src = r#"
#version 400

uniform Matrices {
    mat4 view;
    mat4 projection;
};

in vec3 position;
in vec3 normal;
out vec3 frag_position;
out vec3 frag_normal;

void main() {
    vec4 final_position;
    final_position = projection * view * vec4(position, 1.0);
    frag_position = final_position.xyz;
    frag_normal = normal;
    gl_Position = final_position;
}
"#;

        let frag_src = r#"
#version 400

in vec3 frag_position;
in vec3 frag_normal;
out vec4 color;

void main() {
    color = vec4(frag_normal, 1.0);
}
"#;

        let mut geometry = renderer.create_geometry(vertex_data, index_data, vdesc, IndexType::U32, vert_src, frag_src);

         geometry.update_params(&|params| {
            //params.set("tex", ParamValue::Texture2D(texture.param_handle()));
            //params.set("tex2", ParamValue::Texture2D(texture2.param_handle()));
            let eye = Point3::new(0f32, 0f32, 5f32);
			let pos = Point3::new(0f32, 0f32, 0f32);
			let up = Vector3::new(0f32, 1f32, 0f32);
            
            params.set("view", ParamValue::Mat4(
               	Matrix4::look_at(eye, pos, up)
            ));
            params.set("projection", ParamValue::Mat4(
                perspective(deg(65f32), 1.333f32, 0.1f32, 1000f32) 
            ));
         });

        Game {
            running: true,
            sdl: sdl,
            window: window,
            events: events,
            vid_ctx: vid_ctx,
            gl_ctx: gl_ctx,
            renderer: renderer,
            geometry: geometry,
        }
    }

    fn key_up_event(&mut self, keycode: sdl2::keyboard::Keycode) {
        match keycode {
            sdl2::keyboard::Keycode::Escape => self.running = false,
            _                               => (),
        }
    }

    fn do_window_events(&mut self) {
        let events: Vec<Event> = self.events.poll_iter().collect();
        for ev in events {
            match ev {
                Event::Quit{..}                   => self.running = false,
                Event::KeyUp{keycode, ..}         => self.key_up_event(keycode.unwrap()),
                _                                 => (),
            }
        }
    }

    fn render(&mut self) {
        self.renderer.clear(1.0, 0.3, 0.3, 1.0);

        self.renderer.draw_geometry(&mut self.geometry);

        self.window.gl_swap_window();
    }

    pub fn run(&mut self) {
        while self.running {
            self.do_window_events();
            self.render();

            thread::sleep_ms(1000/60);
        }
    }
}

