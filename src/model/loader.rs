use glam::*;
use crate::model::{Texture, load_texture};
use std::path::Path;

/*
The `Vertex` struct represents a single vertex in a 3D mesh. It includes position and normal
data, which are essential for rendering and lighting calculations. The `Default` trait provides
a default vertex with zeroed position and normal.
*/
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coord: Vec2
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vec3::ZERO,
            normal: Vec3::ZERO,
            tex_coord: Vec2::ZERO
        }
    }
}

/*
The `Mesh` struct represents a collection of vertices and indices forming a 3D object. It
also stores a reference to the material index used for rendering the mesh.
*/
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material_idx: usize
}

/*
The `Material` struct defines the appearance of a mesh using a base color stored as a `Vec4`.
The `Default` trait initializes it with a white color.
*/
#[derive(Clone, Debug)]
pub struct Material {
    pub base_color: Vec4,
    pub base_color_texture: Option<Texture>
}

impl Default for Material {
    fn default() -> Self {
        Material {
            base_color: Vec4::ONE,
            base_color_texture: None
        }
    }
}

/*
The `Model` struct aggregates multiple meshes and their associated materials, representing
a complete 3D object that can be rendered.
*/
#[derive(Clone, Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>
}

/*
Processes a single GLTF node, extracting its meshes and associated materials. This function reads
vertex positions, normals, and indices, and maps them to custom `Mesh` and `Vertex` structs.
It also handles material assignment and updates the `materials` array accordingly.
*/
fn process_node(
    node: &gltf::Node,
    buffers: &[gltf::buffer::Data],
    meshes: &mut Vec<Mesh>,
    materials: &mut [Material],
    file_path: &str
) {
    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            if primitive.mode() == gltf::mesh::Mode::Triangles {
                let reader = primitive.reader(
                    |buffer| Some(&buffers[buffer.index()])
                );

                let positions = {
                    let iter = reader
                        .read_positions()
                        .expect("Failed to process mesh node. (Vertices must have positions)");

                    iter.map(|arr| -> Vec3 { Vec3::from(arr) }).collect::<Vec<_>>()
                };

                let mut vertices: Vec<Vertex> = positions
                    .into_iter()
                    .map(|position| {
                        Vertex {
                             position,
                             ..Default::default()
                        }
                }).collect();

                if let Some(normals) = reader.read_normals() {
                    for (i, normal) in normals.enumerate() {
                        vertices[i].normal = Vec3::from(normal);
                    }
                }

                if let Some(tex_coords) = reader.read_tex_coords(0) {
                    for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                        vertices[i].tex_coord = Vec2::from(tex_coord);
                    }
                }

                let indices = reader
                    .read_indices()
                    .map(|read_indices| {
                        read_indices.into_u32().collect::<Vec<_>>()
                    }).expect("Failed to process mesh node. (Indices are required)");
                
                let prim_material = primitive.material();
                let pbr = prim_material.pbr_metallic_roughness();
                let material_idx = primitive.material().index().unwrap_or(0);

                let material = &mut materials[material_idx];
                material.base_color = Vec4::from(pbr.base_color_factor());
                if let Some(base_color_texture) = pbr.base_color_texture() {
                    if let gltf::image::Source::Uri { uri, .. } = base_color_texture.texture().source().source() {
                        let model_path = Path::new(file_path);
                        let texture_path = model_path.parent().unwrap_or_else(|| Path::new("./")).join(uri);
                        let texture_path_str = texture_path.into_os_string().into_string().unwrap();

                        material.base_color_texture = Some(load_texture(&texture_path_str));
                    }
                }

                meshes.push(Mesh {
                    vertices,
                    indices,
                    material_idx
                });
            }
        }
    }
}

/*
Loads a 3D model from a GLTF file. It parses the document, processes the nodes to extract
meshes and materials, and assembles them into a `Model` struct for further use.
*/
pub fn load_model(file_path: &str) -> Model {
    let (document, buffers, _images) = gltf::import(file_path)
        .expect("Failed to load model.");

    let mut meshes = Vec::new();
    let mut materials = vec![Material::default(); document.materials().len()];
    if materials.is_empty() {
        materials.push(Material::default());
    }
    
    if document.nodes().len() > 0 {
        process_node(
            document.nodes().next().as_ref().unwrap(),
            &buffers,
            &mut meshes,
            &mut materials,
            file_path
        );
    }

    Model {
        meshes,
        materials
    }
}