extern crate glium;

use std::thread;

use renderer;

pub struct Game {
    game_name: String,
    running: bool,
    device: renderer::Device,
}

impl Game {
    pub fn new() -> Game {
        use glium::DisplayBuild;
        
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        Game {
            game_name: "My Super Game!".to_string(),
            running: true,
            device: renderer::Device::new(display),
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
