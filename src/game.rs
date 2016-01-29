extern crate sdl2;
extern crate gl;

use std::thread;
use config::load_config_file;
use self::sdl2::event::{Event};

use renderer::*;
use renderer::backends::{renderer_factory, determine_best_renderer};

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
            Err(x) => panic!(format!("failed to bind window to OpenGL. Reason: {}", x))
        }

        gl::load_with(|name| vid_ctx.gl_get_proc_address(name) as *const _);

        let renderer_name = determine_best_renderer();
        let mut renderer = renderer_factory(&renderer_name).unwrap();

        let mut vdesc = VertexLayoutDescription::new();
        vdesc.add_element("position", VertexElementType::F32F32);

        let vertex_data = BufferData::new_initialized(vec![
            -0.5f32, -0.5,
            0.0,     0.5,
            0.5,     0.0,
        ]);

        //let vbo = renderer.create_vertex_buffer_object(vbo_data).unwrap();

        //let vlayout = renderer.create_vertex_layout(vdesc, vbo).unwrap();

        let index_data = BufferData::new_initialized(vec![
            0u32, 1, 2
        ]);

        //let ibo = renderer.create_index_buffer_object(IndexType::U32, ibo_data).unwrap();

        let vert_src = r#"
#version 140

in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

        let frag_src = r#"
#version 140

out vec4 color;

void main() {
    color = vec4(0.0, 0.0, 1.0, 1.0);
}
"#;

        //let program = renderer.create_program(vert_src.to_string(), frag_src.to_string()).unwrap();

        let geometry = renderer.create_geometry(vertex_data, index_data, vdesc, IndexType::U32, vert_src, frag_src);

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

        //self.renderer.draw(self.vbo, self.ibo, self.program);
        self.renderer.draw_geometry(&self.geometry);

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

