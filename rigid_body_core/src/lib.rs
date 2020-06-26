mod simulation;

pub mod math;
pub mod mesh;
pub mod render;
pub mod utility;

use std::cell::Cell;
pub use simulation::{
    rigid_body,
    RigidBodySimulation,
};
pub use simulation::{
    Contact,
    SeparatingPlane,
};
    
pub type UID = usize;

fn get_new_uid() -> UID {
    thread_local!(static CURRENT_UID: Cell<UID> = Cell::new(0));
    CURRENT_UID.with(|thread_id| {
	let uid = thread_id.get();
	thread_id.set(uid+1);
	uid
    })
}
