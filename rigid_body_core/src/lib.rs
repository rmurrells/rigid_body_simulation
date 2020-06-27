pub mod config;
pub mod input;
pub mod math;
pub mod mesh;
pub mod render;
pub mod utility;
mod simulation;

use math::vector::Vector3d;
use mesh::Mesh;
use input::{
    InputCore,
    camera_mover::{
	CameraMode,
	CameraMover,
    },
};
use render::{
    Camera,
    Color,
    RendererCore,
};
use rigid_body::RigidBody;
use std::cell::Cell;
use utility::FPSManager;
pub use simulation::{
    rigid_body,
    Simulation,
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

pub struct RigidBodySimulationCore {
    pub debug: bool,
    pub input: InputCore,
    pub camera_mover: CameraMover,
    pub renderer: RendererCore,
    simulation: Simulation,
    fps_manager_opt: Option<FPSManager>,
}

impl RigidBodySimulationCore {
    pub fn new(window_size: (u32, u32)) -> Self {
	Self {
	    debug: false,
	    input: InputCore::default(),
	    renderer: RendererCore::new(window_size),
	    simulation: Simulation::default(),
	    camera_mover: CameraMover {
                center: Vector3d::new(5., 5., 5.),
                camera_range: (0., 500.),
                theta_scale: 0.0045,
                wheel_scale: 1.,
                move_fact: 0.1,
                mode: CameraMode::Fps,
            },
	    fps_manager_opt: None,
	}
    }

    pub fn set_window_size(&mut self, window_size: (u32, u32)) {
	self.renderer.set_window_size(window_size);
    }
    
    pub fn tick(&mut self) -> bool {
	self.handle_input();
	self.render();
	if let Some(fps_manager) = &mut self.fps_manager_opt {
	    fps_manager.sleep_to_next_frame();
	}
	!self.input.quit
    }

    fn handle_input(&mut self) {
 	self.camera_mover.move_camera(&self.input, self.renderer.camera_mut());
	if self.input.reset {
	    self.simulation.reset();
	}
	if self.input.advance_simulation || self.input.tick {
	    self.simulation.tick(
		if let Some(fps_manager) = &mut self.fps_manager_opt {
		    fps_manager.frame_duration.as_micros() as f64 / 1_000_000.
		} else {
		    1./60.
		}
	    );
	}
	self.input.clear();
    }
    
    fn render(&mut self) {
	self.renderer.clear(Color::rgb(0, 0, 0));
	if self.debug {
	    self.renderer.render_rigid_bodies_debug(&self.simulation);
	} else {
	    self.renderer.render_rigid_bodies(&self.simulation.rigid_bodies);
	}
    }
}

pub trait GetRigidBodySimulation {
    fn get_rigid_body_simulation(&mut self) -> &mut RigidBodySimulationCore;
}

pub trait RigidBodySimulation: GetRigidBodySimulation {
    fn add_rigid_body(
        &mut self, rigid_body: RigidBody, mesh_opt: Option<(Mesh, Color)>,
    ) {
	let core = self.get_rigid_body_simulation();
        let uid = rigid_body.uid();
        core.simulation.add_rigid_body(rigid_body);
        if let Some((mesh, color)) = mesh_opt {
            core.renderer.add_mesh(uid, mesh, color);
        }
    }

    fn camera_mover_mut(&mut self) -> &mut CameraMover {
	&mut self.get_rigid_body_simulation().camera_mover
    }
    
    fn camera_mut(&mut self) -> &mut Camera {
        self.get_rigid_body_simulation().renderer.camera_mut()
    }

    fn set_fps(&mut self, fps: u64) {
	let core = self.get_rigid_body_simulation();
	if fps == 0 {
	    core.fps_manager_opt = None;
	} else {
	    core.fps_manager_opt = Some(FPSManager::new(fps));
	}
    }
}

impl GetRigidBodySimulation for RigidBodySimulationCore {
    fn get_rigid_body_simulation(&mut self) -> &mut RigidBodySimulationCore {
	self
    }
}
impl RigidBodySimulation for RigidBodySimulationCore {}	
