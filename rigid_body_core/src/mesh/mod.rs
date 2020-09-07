#![allow(dead_code)]
pub mod obj_loader;
pub mod polyhedron_meshes;

use crate::math::vector::Vector3d;

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vector3d>,
    pub mesh_triangles: Vec<MeshTriangle>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vector3d>,
        mesh_triangles: Vec<MeshTriangle>,
    ) -> Self {
        Self {
            vertices,
            mesh_triangles,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MeshTriangle {
    pub vertex_indices: [usize; 3],
    pub normal: Vector3d,
}

impl MeshTriangle {
    pub fn new(vertex_indices: &[usize; 3], normal: &Vector3d) -> Self {
        Self {
            vertex_indices: *vertex_indices,
            normal: *normal,
        }
    }

    pub fn normal(
        vertex_1: &Vector3d,
        vertex_2: &Vector3d,
        vertex_3: &Vector3d,
    ) -> Vector3d {
        vertex_2.sub(vertex_1).cross(&vertex_3.sub(vertex_1)).dir()
    }

    pub fn norm_from_vertices(
        vertices: &[Vector3d],
        vertex_indices: &[usize; 3],
    ) -> Self {
        Self::new(
            vertex_indices,
            &Self::normal(&vertices[0], &vertices[1], &vertices[2]),
        )
    }
}
