mod input;

use rigid_body_core::{
    config, input::camera_mover::CameraMode, render::ScreenBufferTrait,
    RigidBodySimulationCore, RigidBodySimulationCoreAccess,
    RigidBodySimulationTrait,
};

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
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
            rigid_body_simulation_core: RigidBodySimulationCore::new((
                width, height,
            )),
        }
    }

    pub fn on_key(&mut self, button: u32, down: bool) {
        input::key(button, down, &mut self.rigid_body_simulation_core.input);
    }

    pub fn on_mouse_button(&mut self, button: u32, down: bool) {
        input::mouse_button(
            button,
            down,
            &mut self.rigid_body_simulation_core.input,
        );
    }

    pub fn on_mouse_move(&mut self, x: i32, y: i32) {
        input::mouse_move(x, y, &mut self.rigid_body_simulation_core.input);
    }

    pub fn on_mouse_wheel(&mut self, xrel: i32, yrel: i32) {
        input::mouse_wheel(
            xrel,
            yrel,
            &mut self.rigid_body_simulation_core.input,
        );
    }

    pub fn pixel_buffer(&mut self) -> *const u8 {
        self.rigid_body_simulation_core
            .renderer
            .pixel_buffer()
            .as_ptr()
    }

    pub fn tick(&mut self) -> bool {
        self.rigid_body_simulation_core.tick()
    }
}

impl RigidBodySimulationCoreAccess for RigidBodySimulationWAsm {
    fn rigid_body_simulation_core_access(
        &mut self,
    ) -> &mut RigidBodySimulationCore {
        &mut self.rigid_body_simulation_core
    }
}
impl RigidBodySimulationTrait for RigidBodySimulationWAsm {}

#[wasm_bindgen]
pub fn init(width: u32, height: u32, n: usize) -> RigidBodySimulationWAsm {
    set_panic_hook();
    let mut ret = RigidBodySimulationWAsm::new(width, height);
    ret.camera_mover_mut().mode = CameraMode::Rel;
    ret.camera_mover_mut().wheel_scale = 2.;
    config::default(n, &mut ret).expect("config::default");
    ret
}
