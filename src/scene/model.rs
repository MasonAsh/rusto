use renderer::Renderer;
use renderer::shader_params::ParamValue;
use renderer::geometry::Geometry;
use super::transform::Transform;
use super::component::SceneComponent;
use super::camera::Camera;

pub struct Model {
    global_transform: Transform,
    local_transform: Transform,
    geometries: Vec<Box<Geometry>>,
}

impl Model {
    pub fn new(global_transform: &Transform, geometries: Vec<Box<Geometry>>) -> Model {
        Model {
            global_transform: global_transform.clone(),
            local_transform: Transform::identity(),
            geometries: geometries,
        }
    }
}

impl SceneComponent for Model {
    fn global_transform_change(&mut self, transform: &Transform) {
        self.global_transform = transform.clone() + self.local_transform.clone();
    }
    
    fn local_transform(&self) -> &Transform {
        &self.local_transform
    }
    
    fn local_transform_mut(&mut self) -> &mut Transform {
        &mut self.local_transform
    }
    
    fn render(&mut self, renderer: &mut Box<Renderer>, camera: &Camera) {
        let proj = camera.projection();
        let view = camera.view();
        let model = self.global_transform.to_matrix();
        let mvp = proj * view * model;
        
        for geometry in self.geometries.iter_mut() {
            geometry.update_params(&|params| {
                params.set("model_view_proj", ParamValue::Mat4(mvp)); 
            });
            
            renderer.draw_geometry(geometry);
        }
    }
}
