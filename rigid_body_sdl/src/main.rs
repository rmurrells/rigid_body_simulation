use rigid_body_sdl::{
    config::test,
    StrResult,
};

fn main() -> StrResult<()> {
    let mut rigid_body = test::bounding_box()?;
    while rigid_body.tick()?{}
    Ok(())
}
