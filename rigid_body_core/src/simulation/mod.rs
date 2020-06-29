pub mod bounding_box;
pub mod rigid_body;
mod bounding_box_collision_manager;
mod force_manager;
mod collision_manager;
mod collision_table;
mod ode;

pub use collision_manager::SeparatingPlane;
pub use collision_table::Contact;
use crate::math::vector::Vector3d;
use bounding_box::BoundingBox;
use collision_manager::CollisionManager;
use force_manager::ForceManager;
use rigid_body::RigidBody;

#[derive(Default)]
pub struct Simulation {
    pub collision_manager: CollisionManager,
    pub rigid_bodies: Vec<RigidBody>,
    initial_rigid_bodies: Vec<RigidBody>,
    force_manager: ForceManager,
    bounding_box: BoundingBox,
}

impl Simulation {
    pub fn new() -> Self {
	Self::default()
    }
    
    pub fn add_rigid_body(&mut self, rigid_body: RigidBody) {
	self.initial_rigid_bodies.push(rigid_body.clone());
	self.rigid_bodies.push(rigid_body);
	self.collision_manager.generate(&self.rigid_bodies);
    }

    pub fn reset(&mut self) {
	self.rigid_bodies = self.initial_rigid_bodies.clone();
	self.collision_manager.generate(&self.rigid_bodies);
    }

    pub fn bounding_box(&self) -> &BoundingBox {
	&self.bounding_box
    }
    
    pub fn set_bounding_box(
	&mut self, dimensions_opt: &Option<(Vector3d, Vector3d)>,
    ) {
	self.bounding_box.set(
	    dimensions_opt,
	    &mut self.initial_rigid_bodies,
	    &mut self.rigid_bodies,
	);
    }
    
    pub fn tick(&mut self, delta_t: f64) {
	self.force_manager.resultant(&mut self.rigid_bodies);
	ode::euler(delta_t, &mut self.rigid_bodies);
	if let Some(bounding_box) = &self.bounding_box.inner_opt {
	    bounding_box.contain(&mut self.rigid_bodies);
	}
	self.collision_manager.collide_simple(&mut self.rigid_bodies);
	for rigid_body in &mut self.rigid_bodies {
	    rigid_body.clear_forces();
	}
    }
}

