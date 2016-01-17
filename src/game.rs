extern crate glium;

use std::thread;
use renderer;
use std::collections::HashMap;

use config::load_config_file;

pub struct Game {
    game_name: String,
    running: bool,
    device: renderer::Device,
    config: HashMap<String, String>
}

impl Game {
    pub fn new() -> Game {
        use glium::DisplayBuild;
        use glium::glutin::get_primary_monitor;

        let config = load_config_file();

        let width: u32;
        let height: u32;
        let window_title: String;
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

        let mut builder = glium::glutin::WindowBuilder::new();

        builder = builder.with_dimensions(width, height)
            .with_title(window_title);

        if fullscreen {
            builder = builder.with_fullscreen(get_primary_monitor());
        }

        let display = builder.build_glium().unwrap();
        
        Game {
            game_name: "My Super Game!".to_string(),
            running: true,
            device: renderer::Device::new(display),
            config: config,
        }
    }

    fn do_window_events(&mut self) {
        for ev in self.device.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => self.running = false,
                _                            => ()
            }
        }
    }
    
    fn render(&mut self) {
        use glium::Surface;
        
        let mut target = self.device.display.draw();

        target.clear_color(0.0, 0.0, 1.0, 1.0);
        
        target.finish().unwrap();
    }

    pub fn run(&mut self) {
        while self.running {
            self.do_window_events();
            self.render();

            thread::sleep_ms(1000/60);
        }
    }
}
