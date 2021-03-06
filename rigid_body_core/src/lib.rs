pub mod config;
pub mod input;
pub mod math;
pub mod mesh;
pub mod render;
mod simulation;
pub mod utility;

use input::{
    camera_mover::{CameraMode, CameraMover},
    InputCore,
};
use math::vector::Vector3d;
use render::{
    Camera, Color, Draw3dTrait, RenderOption, RendererCore, ScreenBufferTrait,
};
use rigid_body::RigidBody;
pub use simulation::{rigid_body, Contact, SeparatingPlane, Simulation};
use std::cell::Cell;
use utility::FPSManager;

pub type UID = usize;

fn get_new_uid() -> UID {
    thread_local!(static CURRENT_UID: Cell<UID> = Cell::new(0));
    CURRENT_UID.with(|thread_id| {
        let uid = thread_id.get();
        thread_id.set(uid + 1);
        uid
    })
}

pub struct RigidBodySimulationCore {
    pub input: InputCore,
    pub camera_mover: CameraMover,
    pub renderer: RendererCore,
    simulation: Simulation,
    fps_manager_opt: Option<FPSManager>,
}

impl RigidBodySimulationCore {
    pub fn new(window_size: (u32, u32)) -> Self {
        Self {
            input: InputCore::default(),
            renderer: RendererCore::new(window_size),
            simulation: Simulation::default(),
            camera_mover: CameraMover {
                center: Vector3d::default(),
                camera_range: (30., 100.),
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
        self.set_debug(self.input.debug);
        self.camera_mover
            .move_camera(&self.input, self.renderer.camera_mut());
        if self.input.reset {
            self.simulation.reset();
        }
        if self.input.advance_simulation || self.input.tick {
            self.simulation.tick(
                if let Some(fps_manager) = &mut self.fps_manager_opt {
                    fps_manager.frame_duration.as_micros() as f64 / 1_000_000.
                } else {
                    1. / 60.
                },
            );
        }
        self.input.clear();
    }

    fn render(&mut self) {
        self.renderer.clear(Color::rgb(0, 0, 0));
        self.renderer.render_simulation(&self.simulation);
    }
}

pub trait RigidBodySimulationCoreAccess {
    fn rigid_body_simulation_core_access(
        &mut self,
    ) -> &mut RigidBodySimulationCore;
}

pub trait RigidBodySimulationTrait: RigidBodySimulationCoreAccess {
    fn add_rigid_body(
        &mut self,
        rigid_body: RigidBody,
        render_opt: RenderOption,
    ) {
        let core = self.rigid_body_simulation_core_access();
        let uid = rigid_body.uid();
        core.simulation.add_rigid_body(rigid_body);
        core.renderer.set_uid(uid, render_opt);
    }

    fn camera_mover_mut(&mut self) -> &mut CameraMover {
        &mut self.rigid_body_simulation_core_access().camera_mover
    }

    fn camera_mut(&mut self) -> &mut Camera {
        self.rigid_body_simulation_core_access()
            .renderer
            .camera_mut()
    }

    fn set_bounding_box(
        &mut self,
        opt: Option<(&Vector3d, &Vector3d, RenderOption)>,
    ) {
        let core = &mut self.rigid_body_simulation_core_access();
        let simulation = &mut core.simulation;
        match opt {
            Some((min, max, render_opt)) => {
                simulation.set_bounding_box(&Some((min, max)));
                let bounding_box = &simulation.bounding_box();
                core.renderer.set_uid(bounding_box.uid, render_opt);
                for uid in &simulation
                    .bounding_box()
                    .inner_opt
                    .as_ref()
                    .unwrap()
                    .rigid_body_uids
                {
                    core.renderer.set_uid(*uid, RenderOption::Invisible);
                }
            }
            None => simulation.set_bounding_box(&None),
        }
    }

    fn set_debug(&mut self, set: bool) {
        let core = self.rigid_body_simulation_core_access();
        core.simulation.set_debug(set);
        core.renderer.set_debug(set);
    }

    fn set_fps(&mut self, fps: u64) {
        let core = self.rigid_body_simulation_core_access();
        if fps == 0 {
            core.fps_manager_opt = None;
        } else {
            core.fps_manager_opt = Some(FPSManager::new(fps));
        }
    }
}

impl RigidBodySimulationCoreAccess for RigidBodySimulationCore {
    fn rigid_body_simulation_core_access(
        &mut self,
    ) -> &mut RigidBodySimulationCore {
        self
    }
}
impl RigidBodySimulationTrait for RigidBodySimulationCore {}
