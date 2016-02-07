mod component;
mod node;
mod transform;
mod model;
mod camera;

use common::*;
use renderer::Renderer;
use renderer::geometry::Geometry;
use renderer::IndexType;
use renderer::util::mesh::{load_meshes_from_file, MeshData, MeshOptions};

pub use self::node::{Node, NodeRef, WeakNodeRef};
pub use self::component::SceneComponent;
pub use self::model::Model;
pub use self::camera::Camera;

use std::path::Path;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};

pub struct Scene {
    renderer: Box<Renderer>,
    root_node: Rc<RefCell<Node>>,
    camera: Camera,
}

impl Scene {
    pub fn new(renderer: Box<Renderer>, aspect: f32) -> Scene {
        Scene {
            renderer: renderer,
            root_node: Rc::new(RefCell::new(Node::new("_root", None))),
            camera: Camera::new(deg(45f32), aspect, 0.1f32, 1000f32),
        }
    }
    
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    
    pub fn new_child_node(&mut self, name: &str) -> NodeRef {
        let child = Node::new(name, Some(Rc::downgrade(&self.root_node)));
        self.root_node.borrow_mut().attach_child(child)
    }
    
    pub fn attach_model_component_from_file(&mut self, node: &NodeRef, path: &Path) {
        let mut geometries: Vec<Box<Geometry>> = Vec::new();
        
        let mesh_data: Vec<MeshData> = load_meshes_from_file(path, &MeshOptions::default()).unwrap();

        // this lameness will have to suffice until a proper material
        // abstraction is in place.        
        let vert_src = r#"
#version 400

uniform Matrices {
    mat4 model_view_proj;
};
        
in vec3 position;
in vec2 tex_coord;
out vec2 frag_tex_coord;

void main() {
    gl_Position = model_view_proj * vec4(position, 1.0);
}
"#;

let frag_src = r#"
#version 400

uniform sampler2D tex;
in vec2 frag_tex_coord;
out vec4 color;

void main() {
    color = texture(tex, frag_tex_coord);
}
"#;
        
        for mesh_datum in mesh_data {
            let geometry = self.renderer.create_geometry(
                &mesh_datum.vertex_data,
                &mesh_datum.index_data,
                &mesh_datum.layout,
                IndexType::U32,
                vert_src,
                frag_src);

            geometries.push(geometry);
        }
        
        let model = Box::new(Model::new(node.borrow().transform(), geometries));
        node.borrow_mut().attach_component(model);
    }
    
    fn render_nodes_recursive(&mut self, node: NodeRef) {
        for component in node.borrow_mut().components_mut().iter_mut() {
            component.render(&mut self.renderer, &self.camera)
        }
        
        let borrow = node.borrow();
        
        // Not the most rustic code imaginable...
        for i in 0..borrow.child_count() {
            let child = borrow.child_by_index(i);
            
            let child = match child {
                Some(x) => x,
                None    => continue,
            };
            
            self.render_nodes_recursive(child);
        }
    }
    
    pub fn frame(&mut self) {
        self.renderer.clear(1.0, 0.3, 0.3, 1.0);
        
        let root = self.root_node.clone();
        self.render_nodes_recursive(root);
    }
}
