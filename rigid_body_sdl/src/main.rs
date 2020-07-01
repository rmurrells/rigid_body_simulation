use rigid_body_sdl::{
    config,
    RigidBodySimulationSDL,
    StrResult,
};

fn main() -> StrResult<()> {
    let mut rigid_body_simulation = RigidBodySimulationSDL::new((800, 600))?;
    println!("Loading simulation...");
    config::default(8, &mut rigid_body_simulation)?;
    println!("Done.");
    while rigid_body_simulation.tick()?{}
    Ok(())
}
