use super::geometry::*;

extern crate glium;

pub struct Device {
    pub display: glium::Display,
    pub geometry: Vec<Geometry>,
}

impl Device {
    pub fn new(display: glium::Display) -> Device {
        Device {
            display: display,
            geometry: Vec::new(),
        }
    }
    
    pub fn add_geometry(&mut self, geom: Geometry) {
        self.geometry.push(geom);
    }
}
