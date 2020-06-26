mod fps_manager;
mod input;
mod render;

pub mod config;

use fps_manager::FPSManager;
use input::{
    camera_mover::{
	CameraMode,
	CameraMover,
    },
    Input,
    InputState,
};
use render::RendererSDL;
use rigid_body_core::{
    math::vector::Vector3d,
    mesh::Mesh,
    render::{
	Camera,
	Color,
	Renderer,
    },
    rigid_body::RigidBody,
    RigidBodySimulation,
};
use sdl2::Sdl;

pub use rigid_body_core::{
    math,
    mesh,
};
pub use render::StrResult;

pub struct RigidBodySimulationSDL {
    pub debug: bool,
    advance_simulation: bool,
    tick: bool,
    simulation: RigidBodySimulation,
    renderer: RendererSDL,
    input: Input,
    camera_mover: CameraMover,
    fps_manager_opt: Option<FPSManager>,
    _context: Sdl,
}

impl RigidBodySimulationSDL {
    pub fn new() -> StrResult<Self> {
	let context = sdl2::init()?;
	Ok(Self {
	    debug: false,
	    advance_simulation: true,
	    tick: true,
	    simulation: RigidBodySimulation::new(),
	    renderer: RendererSDL::new(
		&context, "RigidBodySimulationSDL", (800, 600),
	    )?,
	    input: Input::new(&context)?,
	    camera_mover: CameraMover {
		center: Vector3d::new(5., 5., 5.),
		camera_range: (0., 500.),
		theta_scale: 0.0045,
		wheel_scale: 1.,
		move_fact: 0.1,
		mode: CameraMode::Fps,
	    },
	    fps_manager_opt: Some(FPSManager::new(60)),
	    _context: context,
	})
    }

    pub fn add_rigid_body(
	&mut self, rigid_body: RigidBody, mesh_opt: Option<(Mesh, Color)>,
    ) {
	let uid = rigid_body.uid();
	self.simulation.add_rigid_body(rigid_body);
	if let Some((mesh, color)) = mesh_opt {
	    self.renderer.add_mesh(uid, mesh, color);
	}
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
	self.renderer.camera_mut()
    }
    
    pub fn set_fps(&mut self, fps: u64) {
	if fps == 0 {
	    self.fps_manager_opt = None;
	} else {
	    self.fps_manager_opt = Some(FPSManager::new(fps));
	}
    }
    
    pub fn tick(&mut self) -> StrResult<bool> {
	self.tick = false;
	match self.input.get() {
	    InputState::Continue => (),
	    InputState::Pause =>
		self.advance_simulation = !self.advance_simulation,
	    InputState::Quit => return Ok(false),
	    InputState::Reset => self.simulation.reset(),
	    InputState::Tick => self.tick = true,
	}
	self.camera_mover.move_camera(&self.input, self.renderer.camera_mut());
	if self.advance_simulation || self.tick {
	    self.simulation.tick(
		if let Some(fps_manager) = &mut self.fps_manager_opt {
		    fps_manager.frame_duration.as_micros() as f64 / 1_000_000.
		} else {
		    1./60.
		}
	    );
	}
	self.render()?;
	if let Some(fps_manager) = &mut self.fps_manager_opt {
	    fps_manager.sleep_to_next_frame();
	}
	Ok(true)
    }

    fn render(&mut self) -> StrResult<()> {
	self.renderer.clear((0, 0, 0));
	if self.debug {
	    self.renderer.render_rigid_bodies_debug(&self.simulation);
	} else {
	    self.renderer.render_rigid_bodies(&self.simulation.rigid_bodies);
	}
	self.renderer.present()
    }
}
