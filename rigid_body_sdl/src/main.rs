use rigid_body_sdl::{
    config::test,
    CameraMode,
    RigidBodySimulationSDL,
    RigidBodySimulationTrait,
    StrResult,
};

fn main() -> StrResult<()> {
    let mut rigid_body_simulation = RigidBodySimulationSDL::new((800, 600))?;
    rigid_body_simulation.camera_mover_mut().mode = CameraMode::Rel;
    test::bounding_box(&mut rigid_body_simulation);
    while rigid_body_simulation.tick()?{}
    Ok(())
}
