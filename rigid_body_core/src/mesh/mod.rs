#![allow(dead_code)]
pub mod obj_loader;
pub mod polyhedron_meshes;

use crate::math::{
    triangle::Triangle3d,
    vector::Vector3d,
};

#[derive(Clone, Default)]
pub struct Mesh {
    pub mesh_triangles: Vec<MeshTriangle>,
}

impl Mesh {
    pub fn new() -> Self {
	Self {
	    mesh_triangles: Vec::new(),
	}
    }

    pub fn add(&mut self, mesh_triangle: &MeshTriangle) {
	self.mesh_triangles.push(*mesh_triangle);
    }
}

impl From<Vec<MeshTriangle>> for Mesh {
    fn from(mesh_triangles: Vec<MeshTriangle>) -> Self {
	Self {
	    mesh_triangles
	}
    }
}

#[derive(Clone, Copy)]
pub struct MeshTriangle {
    pub triangle_3d: Triangle3d,
    pub normal: Vector3d,
}

impl MeshTriangle {
    pub fn new(triangle_3d: &Triangle3d, normal: &Vector3d) -> Self {
	Self {
	    triangle_3d: *triangle_3d,
	    normal: *normal,
	}
    }
    
    pub fn norm_from_vertices(
	vertex_1: &Vector3d,
	vertex_2: &Vector3d,
	vertex_3: &Vector3d,
    ) -> Self {
	Self::new(
	    &Triangle3d::new(
		vertex_1, vertex_2, vertex_3,
	    ),
	    &vertex_2.sub(vertex_1)
		.cross(&vertex_3.sub(vertex_1))
		.dir(),
	)
    }
}
