mod input;
mod render;

use input::InputSDL;
use render::RendererSDL;
use rigid_body_core::{
    GetRigidBodySimulation,
    RigidBodySimulationCore,
};
use sdl2::Sdl;

pub use rigid_body_core::{
    config,
    math,
    mesh,
    rigid_body::RigidBody,
    RigidBodySimulation,
};
pub use render::StrResult;

pub struct RigidBodySimulationSDL {
    rigid_body_simulation_core: RigidBodySimulationCore,
    renderer: RendererSDL,
    input: InputSDL,
    _context: Sdl,
}

impl RigidBodySimulationSDL {
    pub fn new(window_size: (u32, u32)) -> StrResult<Self> {
	let context = sdl2::init()?;
	let mut ret = Self {
	    rigid_body_simulation_core: RigidBodySimulationCore::new(
		window_size,
	    ),
	    renderer: RendererSDL::new(
		&context, "RigidBodySimulationSDL", window_size,
	    )?,
	    input: InputSDL::new(&context)?,
	    _context: context,
	};
	ret.set_fps(60);
	Ok(ret)
    }

    pub fn set_window_size(
	&mut self, window_size: (u32, u32),
    ) -> StrResult<()> {
	self.renderer.set_window_size(window_size)
	    .map_err(|e| e.to_string())?;
	self.rigid_body_simulation_core.set_window_size(window_size);
	Ok(())
    }
    
    pub fn tick(&mut self) -> StrResult<bool> {
	self.input.get(&mut self.rigid_body_simulation_core.input);
	if !self.rigid_body_simulation_core.tick() {return Ok(false);}
	self.renderer.present(
	    self.rigid_body_simulation_core.renderer.get_data_mut(),
	)?;
	Ok(true)
    }
}

impl GetRigidBodySimulation for RigidBodySimulationSDL {
    fn get_rigid_body_simulation(&mut self) -> &mut RigidBodySimulationCore {
	&mut self.rigid_body_simulation_core
    }
}
impl RigidBodySimulation for RigidBodySimulationSDL {}
