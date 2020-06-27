#![allow(dead_code)]
use crate::math::vector::Vector3d;
use super::rigid_body::RigidBody;

pub type ForceFn = fn(&RigidBody, &Vector3d) -> (Vector3d, Vector3d);

pub struct ForceManager {
    force_functions: Vec<ForceFn>,
}

impl ForceManager {
    pub fn new(force_functions: Vec<ForceFn>) -> Self {
        Self {
            force_functions,
        }
    }

    pub fn resultant(&self, rigid_bodies: &mut [RigidBody]) {
	for rigid_body in rigid_bodies {
	    if rigid_body.is_immovable() {continue;}
	    self.resultant_impl(rigid_body);
	}
    }

    fn resultant_impl(&self, rigid_body: &mut RigidBody) {
        for force_function in &self.force_functions {
	    let (force, torque) =
		force_function(rigid_body, rigid_body.velocity());
            rigid_body.force.add_assign(&force);
	    rigid_body.torque.add_assign(&torque);
        }
    }
}

impl Default for ForceManager {
    fn default() -> Self {
	Self::new(Vec::new())
    }
}

pub fn earth_gravity(
    rigid_body: &RigidBody, _velocity: &Vector3d,
) -> (Vector3d, Vector3d) {
    (Vector3d::new(0., -9.81/rigid_body.mass_inv(), 0.), Vector3d::default())
}

pub fn drag(
    _rigid_body: &RigidBody, velocity: &Vector3d,
) -> (Vector3d, Vector3d) {
    (velocity.scale(-0.1), Vector3d::default())
}
