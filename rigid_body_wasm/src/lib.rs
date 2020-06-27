mod input;

use rigid_body_core::{
    config::test,
    GetRigidBodySimulation,
    RigidBodySimulation,
    RigidBodySimulationCore,
};

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct RigidBodySimulationWAsm {
    rigid_body_simulation_core: RigidBodySimulationCore,
}

#[wasm_bindgen]
impl RigidBodySimulationWAsm {
    pub fn new(width: u32, height: u32) -> Self {
	Self {
	    rigid_body_simulation_core: RigidBodySimulationCore::new(
		(width, height),
	    ),
	}
    }

    pub fn on_key(&mut self, button: u32, down: bool) {
	input::key(button, down, &mut self.rigid_body_simulation_core.input);
    }

    pub fn on_mouse_button(&mut self, button: u32, down: bool) {
	input::mouse_button(
	    button, down, &mut self.rigid_body_simulation_core.input,
	);
    }

    pub fn on_mouse_move(&mut self, x: i32, y: i32) {
	input::mouse_move(x, y, &mut self.rigid_body_simulation_core.input);
    }
    
    pub fn pixel_buffer(&mut self) -> *const u8 {
	self.rigid_body_simulation_core.renderer.pixel_buffer().as_ptr()
    }
    
    pub fn tick(&mut self) -> bool {
	self.rigid_body_simulation_core.tick()
    }
}

impl GetRigidBodySimulation for RigidBodySimulationWAsm {
    fn get_rigid_body_simulation(&mut self) -> &mut RigidBodySimulationCore {
	&mut self.rigid_body_simulation_core
    }
}
impl RigidBodySimulation for RigidBodySimulationWAsm {}

#[wasm_bindgen]
pub fn init(width: u32, height: u32) -> RigidBodySimulationWAsm {
    let mut ret = RigidBodySimulationWAsm::new(width, height);
    test::bounding_box(&mut ret);
    ret
}
