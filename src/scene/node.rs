use super::component::SceneComponent;
use super::transform::Transform;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    name: String,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    components: Vec<Box<SceneComponent>>,
    transform: Transform,
}

pub type WeakNodeRef = Weak<RefCell<Node>>;
pub type NodeRef = Rc<RefCell<Node>>;

impl Node {
    pub fn new(name: &str, parent: Option<WeakNodeRef>) -> Node {
        return Node {
            name: name.clone().to_string(),
            parent: parent,
            children: Vec::new(),
            components: Vec::new(),
            transform: Transform::identity(),
        }
    }
    
    pub fn transform(&self) -> &Transform {
        &self.transform
    }
    
    pub fn transform_change(&mut self, fun: &Fn(&mut Transform)) {
        fun(&mut self.transform);
        
        for component in self.components.iter_mut() {
            component.global_transform_change(&self.transform);
        }
    }
    
    pub fn attach_child(&mut self, node: Node) -> NodeRef {
        let rc = Rc::new(RefCell::new(node));
        let copy = rc.clone();
        self.children.push(rc);
        copy
    }
    
    pub fn get_child_by_name(&self, name: &str, recursive: bool) -> Option<NodeRef> {
        let name_str = name.to_string();
        for child in self.children.iter() {
            if child.borrow().name == name_str {
                return Some(child.clone());
            }
            
            if recursive {
                match child.borrow().get_child_by_name(name, recursive) {
                    Some(x) => return Some(x.clone()),
                    None    => (),
                }
            }
        }
        
        None
    }
    
    pub fn components(&self) -> &Vec<Box<SceneComponent>> {
        &self.components
    }
    
    pub fn components_mut(&mut self) -> &mut Vec<Box<SceneComponent>> {
        &mut self.components
    }
    
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
    
    pub fn child_by_index(&self, index: usize) -> Option<NodeRef> {
        match self.children.get(index) {
            Some(x) => Some(x.clone()),
            None    => None,
        }
    }
    
    pub fn attach_component(&mut self, component: Box<SceneComponent>) {
        self.components.push(component);
    }
}
