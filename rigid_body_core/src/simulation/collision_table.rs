use crate::math::{
    geometry,
    vector::Vector3d,
};
use super::rigid_body::RigidBody;
use std::mem;

#[derive(Default)]
pub struct CollisionTable {
    data: Vec<Vec<CollisionStatus>>,
}

impl CollisionTable {
    pub fn new() -> Self {
	Self::default()
    }

    pub fn reset_colliding(&mut self) {
	for i in 1..self.data.len() {
	    for j in 0..i {
		self.data[i][j].colliding = false;
	    }
	}
    }

    pub fn generate(&mut self, n: usize) {
	self.data.clear();
	for i in (0..n).rev() {
	    self.data.push(vec![CollisionStatus::new(); n-i]);
	}
    }

    pub fn get(&self, mut i: usize, mut j: usize) -> &CollisionStatus {
	if i < j {mem::swap(&mut i, &mut j);}
	&self.data[i][j]
    }

    pub fn get_mut(&mut self, mut i: usize, mut j: usize) -> &mut CollisionStatus {
	if i < j {mem::swap(&mut i, &mut j);}
	&mut self.data[i][j]
    }

    pub fn len(&self) -> usize {
	self.data.len()
    }
}

#[derive(Clone)]
pub struct CollisionStatus {
    pub bounding_box: [bool; 3],
    pub separating_plane: SeparatingPlane,
    pub colliding: bool,
    pub contacts: Contacts,
}

impl CollisionStatus {
    fn new() -> Self {
	Self {
	    bounding_box: [false; 3],
            separating_plane: SeparatingPlane::None,
            colliding: false,
	    contacts: Contacts::new(),
	}
    }

    pub fn bounding_box_collision(&self) -> bool {
	self.bounding_box[0] && self.bounding_box[1] && self.bounding_box[2]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SeparatingPlane {
    Face{face_indices: FaceIndices},
    Edge{edge_indices: EdgeIndices},
    None,
}

#[derive(Clone, Copy, Debug)]
pub struct FaceIndices {
    pub face_rigid_body: usize,
    pub face: usize,
    pub face_position: usize,
    pub other_rigid_body: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct EdgeIndices {
    pub plane_rigid_body: usize,
    pub plane_edge: usize,
    pub plane_position: usize,
    pub other_rigid_body: usize,
    pub other_edge: usize,
}

impl EdgeIndices {
    pub fn plane_direction(
	&self, rigid_bodies: &[RigidBody],
    ) -> Option<Vector3d> {
	let plane_rigid_body = &rigid_bodies[self.plane_rigid_body];
	let plane_polyhedron = plane_rigid_body.polyhedron_world();
	let mut plane_direction = plane_polyhedron.edges()[self.plane_edge]
	    .direction().cross(
		rigid_bodies[self.other_rigid_body].polyhedron_world()
		    .edges()[self.other_edge]
		    .direction(),
	    );
	if plane_direction.is_zero() {return None;}
	if geometry::pos_raw_plane_signed_dist(
	    &plane_rigid_body.position,
	    &plane_polyhedron.vertices()[self.plane_position],
	    &plane_direction,
	) > 0. {
	    plane_direction.scale_assign(-1.); 
	}
	plane_direction.normalize();
	Some(plane_direction)
    }
}

pub type Contacts = Vec<Contact>;

#[derive(Clone, Copy)]
pub enum Contact {
    VertexFace{vertex_face_indices: VertexFaceIndices},
    EdgeEdge{edge_edge_indices: EdgeEdgeIndices},
}

#[derive(Clone, Copy, Debug)]
pub struct VertexFaceIndices {
    pub vertex_rigid_body: usize,
    pub vertex: usize,
    pub face_rigid_body: usize,
    pub face: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct EdgeEdgeIndices {
    pub plane_rigid_body: usize,
    pub plane_edge: usize,
    pub other_rigid_body: usize,
    pub other_edge: usize,
    pub contact_position: Vector3d,
    pub plane_direction: Vector3d,
}
