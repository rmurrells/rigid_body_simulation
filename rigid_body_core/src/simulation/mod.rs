mod bounding_box_collision_manager;
mod force_manager;
mod collision_manager;
mod collision_table;
mod ode;
pub mod rigid_body;

use collision_manager::CollisionManager;
use force_manager::ForceManager;
use rigid_body::RigidBody;
pub use collision_manager::SeparatingPlane;
pub use collision_table::Contact;

pub struct RigidBodySimulation {
    pub collision_manager: CollisionManager,
    pub rigid_bodies: Vec<RigidBody>,
    initial_rigid_bodies: Vec<RigidBody>,
    force_manager: ForceManager,
}

impl RigidBodySimulation {
    pub fn new() -> Self {
	Self {
	    force_manager: ForceManager::new(Vec::new()),
	    collision_manager: CollisionManager::new(),
	    initial_rigid_bodies: Vec::new(),
	    rigid_bodies: Vec::new(),
	}
    }

    pub fn add_rigid_body(&mut self, rigid_body: RigidBody) {
	self.rigid_bodies.push(rigid_body.clone());
	self.collision_manager.generate(&self.rigid_bodies);
	self.initial_rigid_bodies.push(rigid_body);
    }

    pub fn reset(&mut self) {
	self.rigid_bodies = self.initial_rigid_bodies.clone();
	self.collision_manager.generate(&self.rigid_bodies);
    }
    
    pub fn tick(&mut self, delta_t: f64) {
	self.force_manager.resultant(&mut self.rigid_bodies);
	ode::euler(delta_t, &mut self.rigid_bodies);
	self.collision_manager.collide_simple(&mut self.rigid_bodies);
	for rigid_body in &mut self.rigid_bodies {
	    rigid_body.clear_forces();
	}
    }
}
