use rigid_body_sdl::{
    config::test,
    RigidBodySimulationSDL,
    StrResult,
};

fn main() -> StrResult<()> {
    let mut rigid_body_simulation = RigidBodySimulationSDL::new((800, 600))?;
    test::bounding_box(&mut rigid_body_simulation);
    while rigid_body_simulation.tick()?{}
    Ok(())
}
