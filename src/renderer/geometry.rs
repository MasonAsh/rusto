use super::vertex_layout::VertexLayoutDescription;
use super::shader_params::ShaderParams;

pub trait Geometry {
    fn get_vertex_layout_description(&self) -> &VertexLayoutDescription;
    fn get_params(&self) -> &ShaderParams;
    fn get_mut_params(&mut self) -> &mut ShaderParams;
    
    /// Convenience function for updating the parameters using a closure.
    /// Example:
    /// `geometry.update_params(&|params| { params.set("whatever", ParamValue::F32(1.0)) });
    fn update_params(&mut self, closure: &Fn(&mut ShaderParams)) {
        let params = self.get_mut_params();
        closure(params);
    }
}
