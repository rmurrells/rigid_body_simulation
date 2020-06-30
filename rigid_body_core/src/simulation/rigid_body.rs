use crate::{
    math::{
	matrix::Matrix3x3,
	matrix_vector,
	moment_of_inertia,
	polyhedron::{
	    Edge,
	    Polyhedron,
	},
	Quarternion,
	vector::Vector3d,
    },
    mesh::Mesh,
    UID,
};
use std::f64::{
    EPSILON,
    MAX,
    MIN,
};

pub type BoundingBox = [Vector3d; 2];

#[derive(Clone)]
pub struct RigidBody {
    uid: UID,
    mass_inv: f64,
    inertia_body: Matrix3x3,
    inertia_body_inv: Matrix3x3,
    polyhedron_body: Polyhedron,
    
    pub position: Vector3d,
    pub quarternion: Quarternion,
    pub momentum: Vector3d,
    pub angular_momentum: Vector3d,

    rotation: Matrix3x3,
    inertia: Matrix3x3,
    inertia_inv: Matrix3x3,
    velocity: Vector3d,
    angular_velocity: Vector3d,
    polyhedron_world: Polyhedron,
    bounding_box: BoundingBox,
    
    pub force: Vector3d,
    pub torque: Vector3d,
}

impl RigidBody {
    pub fn new(
	mass_inv: f64,
	inertia_body_inv: &Matrix3x3,
	polyhedron: Polyhedron,
	position: &Vector3d,
	rotation: &Matrix3x3,
	momentum: &Vector3d,
	angular_momentum: &Vector3d,
    ) -> Self {
	let mut ret = Self {
	    uid: crate::get_new_uid(),
	    mass_inv,
	    inertia_body: if inertia_body_inv.is_zero() {
		*inertia_body_inv
	    } else {
		inertia_body_inv.inverse().expect("inertia_body")
	    },
	    inertia_body_inv: *inertia_body_inv,
	    polyhedron_body: polyhedron.clone(),
	    
	    position: *position,
	    quarternion: Quarternion::from_matrix(&rotation),
	    momentum: *momentum,
	    angular_momentum: *angular_momentum,

	    rotation: *rotation,
	    inertia: Matrix3x3::default(),
	    inertia_inv: Matrix3x3::default(),
	    velocity: Vector3d::default(),
	    angular_velocity: Vector3d::default(),
	    polyhedron_world: polyhedron,
	    bounding_box: [Vector3d::default(), Vector3d::default()],
	    
	    force: Vector3d::default(),
	    torque: Vector3d::default(),
	};
	ret.update_rotation();
	ret.update();
	ret
    }

    pub fn cuboid(
	dimensions: &Vector3d,
	mass_inv: f64,
	position: &Vector3d,
	rotation: &Matrix3x3,
	momentum: &Vector3d,
	angular_momentum: &Vector3d,
    ) -> Self {
	Self::new(
	    mass_inv,
	    &if mass_inv >= EPSILON {
		moment_of_inertia::aligned_cuboid(
		    dimensions, 1./mass_inv,
		).inverse().expect(
		    "cuboid - inertia_body inverse",
		)
	    } else {
		Matrix3x3::default()
	    },
	    Polyhedron::cuboid(dimensions),
	    position,
	    rotation,
	    momentum,
	    angular_momentum,   
	)
    }

    pub fn from_mesh(
	mesh: &Mesh,
	mass_inv: f64,
	inertia_body_inv: &Matrix3x3,
	position: &Vector3d,
	rotation: &Matrix3x3,
	momentum: &Vector3d,
	angular_momentum: &Vector3d,
    ) -> Self {
	let mesh_len = mesh.mesh_triangles.len();
	let mut vertices = Vec::with_capacity(mesh_len*3);
	let mut edges = Vec::with_capacity(mesh_len*3);
	let mut face_vertex_indices = Vec::with_capacity(mesh_len);
	for mesh_triangle in &mesh.mesh_triangles {
	    let mut indices = Vec::with_capacity(3);
	    for vertex in &mesh_triangle.triangle_3d.vertices {
		vertices.push(*vertex);
		indices.push(vertices.len()-1);
	    }
	    let len = vertices.len();
	    edges.push(Edge::new(len-1, len-2, &vertices));
	    edges.push(Edge::new(len-2, len-3, &vertices));
	    edges.push(Edge::new(len-3, len-1, &vertices));
	    face_vertex_indices.push(indices);
	}
	Self::new(
	    mass_inv,
	    inertia_body_inv,
	    Polyhedron::new(
		edges, vertices, face_vertex_indices,
	    ),
	    position,
	    rotation,
	    momentum,
	    angular_momentum,
	)
    }
    
    pub fn angular_velocity(&self) -> &Vector3d {
	&self.angular_velocity
    }

    pub fn bounding_box(&self) -> &BoundingBox {
	&self.bounding_box
    }

    pub fn inertia_inv(&self) -> &Matrix3x3 {
	&self.inertia_inv
    }

    pub fn is_immovable(&self) -> bool {
	self.mass_inv < EPSILON
    }

    pub fn ki_total(&self) -> f64 {
	self.ki_translational()+self.ki_rotational()
    }
    
    pub fn ki_translational(&self) -> f64 {
	if self.is_immovable() {0.}
	else {0.5*self.velocity.mag_sq()/self.mass_inv}
    }

    pub fn ki_rotational(&self) -> f64 {
	if self.is_immovable() {0.}
	else {
	    let mut ret = 0.;
	    for i in 0..3 {
		for j in 0..3 {
		    ret += self.angular_velocity[i]*self.inertia[i][j]
			*self.angular_velocity[j];
		}
	    }
	    ret*0.5
	}
    }
    
    pub fn mass_inv(&self) -> f64 {
	self.mass_inv
    }
    
    pub fn polyhedron_body(&self) -> &Polyhedron {
	&self.polyhedron_body
    }

    pub fn polyhedron_world(&self) -> &Polyhedron {
	&self.polyhedron_world
    }
    
    pub fn rotation(&self) -> &Matrix3x3 {
	&self.rotation
    }

    pub fn uid(&self) -> UID {
	self.uid
    }
    
    pub fn update_angular_velocity(&mut self) {
	self.angular_velocity =
	    matrix_vector::mult_3(&self.inertia_inv, &self.angular_momentum);
    }

    pub fn update_geometry(&mut self) {
	self.bounding_box = [
	    Vector3d::new(MAX, MAX, MAX),
	    Vector3d::new(MIN, MIN, MIN),
	];
	let vertices_body = self.polyhedron_body.vertices();
	let vertices_world = self.polyhedron_world.vertices_mut();
	for i in 0..vertices_world.len() {
	    let vertex = &mut vertices_world[i];
	    *vertex = matrix_vector::mult_3(
		&self.rotation,
		&vertices_body[i],
	    ).add(&self.position);
	    for j in 0..3 {
		if vertex[j] < self.bounding_box[0][j] {
		    self.bounding_box[0][j] = vertex[j];
		}
		if vertex[j] > self.bounding_box[1][j] {
		    self.bounding_box[1][j] = vertex[j];
		}
 	    }
	}
	self.polyhedron_world.update();
    }
    
    pub fn update_velocity(&mut self) {
	self.velocity = self.momentum.scale(self.mass_inv);
    }
    
    pub fn velocity(&self) -> &Vector3d {
	&self.velocity
    }
    
    pub fn clear_forces(&mut self) {
	self.force = Vector3d::default();
	self.torque = Vector3d::default();
    }

    pub fn set_rotation(&mut self, rotation: &Matrix3x3) {
	self.quarternion = Quarternion::from_matrix(&rotation);
	self.update_angular();
    }
    
    pub fn update(&mut self) {
	self.update_velocity();
	self.update_angular();
	self.update_geometry();
    }

    fn update_angular(&mut self) {
	self.update_angular_velocity();
	self.update_rotation();
	self.update_angular_momentum();
    }

    fn update_angular_momentum(&mut self) {
	self.angular_momentum =
	    matrix_vector::mult_3(&self.inertia, &self.angular_velocity);
    }

    fn update_rotation(&mut self) {
	self.quarternion.normalize();
	self.rotation = self.quarternion.to_matrix();
	self.inertia =
	    self.rotation.mult(&self.inertia_body).mult_t(&self.rotation);	
	self.inertia_inv =
	    self.rotation.mult(&self.inertia_body_inv).mult_t(&self.rotation);
    }
}

pub fn ki_translational(rigid_bodies: &[RigidBody]) -> f64 {
    let mut ret = 0.;
    for rigid_body in rigid_bodies {
	ret += rigid_body.ki_translational();
    }
    ret
}

pub fn ki_rotational(rigid_bodies: &[RigidBody]) -> f64 {
    let mut ret = 0.;
    for rigid_body in rigid_bodies {
	ret += rigid_body.ki_rotational();
    }
    ret
}

pub fn ki_total(rigid_bodies: &[RigidBody]) -> f64 {
    let mut ret = 0.;
    for rigid_body in rigid_bodies {
	ret += rigid_body.ki_total();
    }
    ret
}
