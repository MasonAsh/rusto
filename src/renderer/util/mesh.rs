use common::*;

use assimp::Importer;

use std::path::Path;

use renderer::buffer::BufferData;
use renderer::vertex_layout::{VertexLayoutDescription, VertexElementType};

pub struct MeshOptions {
    gen_normals: bool,
    gen_uv_coords: bool,
    calc_tangents: bool,
    position_attr_name: String,
    normal_attr_name: String,
    tex_coord_attr_name: String,
    tangent_attr_name: String,
    bitangent_attr_name: String,
}

impl MeshOptions {
    pub fn default() -> MeshOptions {
        return MeshOptions {
            gen_normals: true,
            gen_uv_coords: false,
            calc_tangents: true,
            position_attr_name: "position".to_string(),
            normal_attr_name: "normal".to_string(),
            tex_coord_attr_name: "tex_coord".to_string(),
            tangent_attr_name: "tangent".to_string(),
            bitangent_attr_name: "bitangent".to_string(),
        }
    }
}

pub struct MeshData {
    pub vertex_data: BufferData,
    pub index_data: BufferData,
    pub layout: VertexLayoutDescription,
}

pub fn load_meshes_from_file(path: &Path, options: &MeshOptions) -> Result<Vec<MeshData>, String> {
    let mut importer = Importer::new();

    importer.triangulate(true);

    if options.gen_normals {
        importer.generate_normals(|x| {
            x.enable = true;
            x.smooth = true;
        });
    }

    if options.gen_uv_coords {
        importer.gen_uv_coords(true);
    }

    if options.calc_tangents {
        importer.calc_tangent_space(|x| x.enable = true);
    }

    let scene = importer.read_file(path.to_str().unwrap()).unwrap();

    let mut result: Vec<MeshData> = Vec::new();

    for mesh in scene.mesh_iter() {
        let mut layout = VertexLayoutDescription::new();

        let positions: Vec<Vec3f> = mesh.vertex_iter().map(|pos| {
            Vec3f::new(pos.x, pos.y, pos.z)
        }).collect();

        layout.add_element(options.position_attr_name.clone(), VertexElementType::F32F32F32);

        let mut normals: Vec<Vec3f> = Vec::new();

        if mesh.has_normals() {
            normals = mesh.normal_iter().map(|norm| {
                Vec3f::new(norm.x, norm.y, norm.z)
            }).collect();

            layout.add_element(options.normal_attr_name.clone(), VertexElementType::F32F32F32);
        }

        let mut tex_coords: Vec<Vec2f> = Vec::new();

        // TODO: Multiple texture coord and color channels

        if mesh.has_texture_coords(0) {
            tex_coords = mesh.texture_coords_iter(0).map(|coord| {
                Vec2f::new(coord.x, coord.y)
            }).collect();

            layout.add_element(options.tex_coord_attr_name.clone(), VertexElementType::F32F32);
        }

        let mut tangents: Vec<Vec3f> = Vec::new();
        let mut bitangents: Vec<Vec3f> = Vec::new();

        if mesh.has_tangents_and_bitangents() {
            tangents = mesh.tangent_iter().map(|tang| {
                Vec3f::new(tang.x, tang.y, tang.z)
            }).collect();

            bitangents = mesh.bitangent_iter().map(|bitang| {
                Vec3f::new(bitang.x, bitang.y, bitang.z)
            }).collect();

            layout.add_element(options.tangent_attr_name.clone(), VertexElementType::F32F32F32);
            layout.add_element(options.bitangent_attr_name.clone(), VertexElementType::F32F32F32);
        }

        let mut interleaved: Vec<f32> = Vec::new();

        for i in 0..(mesh.num_vertices as usize) {
            interleaved.push(positions[i].x);
            interleaved.push(positions[i].y);
            interleaved.push(positions[i].z);

            if mesh.has_normals() {
                interleaved.push(normals[i].x);
                interleaved.push(normals[i].y);
                interleaved.push(normals[i].z);
            }

            if mesh.has_texture_coords(0) {
                interleaved.push(tex_coords[i].x);
                interleaved.push(tex_coords[i].y);
            }

            if mesh.has_tangents_and_bitangents() {
                interleaved.push(tangents[i].x);
                interleaved.push(tangents[i].y);
                interleaved.push(tangents[i].z);
                interleaved.push(bitangents[i].x);
                interleaved.push(bitangents[i].y);
                interleaved.push(bitangents[i].z);
            }
        }
        
        let vertex_data = BufferData::new_initialized(interleaved);

        let mut indices: Vec<u32> = Vec::new();

        for face in mesh.face_iter() {
            indices.push(face[0]);
            indices.push(face[1]);
            indices.push(face[2]);
        }

        let index_data = BufferData::new_initialized(indices);
        
        result.push(MeshData {
            vertex_data: vertex_data,
            index_data: index_data,
            layout: layout,
        });
    }

    Ok(result)
}
