use rigid_body_core::{
    math::{matrix::Matrix3x3, vector::Vector3d},
    rigid_body::RigidBody,
    Simulation,
};
use std::f64::consts::PI;

fn main() -> Result<(), String> {
    let mut rbs = Simulation::new();
    rbs.add_rigid_body(RigidBody::cuboid(
        &Vector3d::new(5., 5., 5.),
        1.,
        &Vector3d::new(0., 0., 0.),
        &Matrix3x3::identity(),
        &Vector3d::new(
            20. * (30. * PI / 180.).cos(),
            20. * (30. * PI / 180.).sin(),
            0.,
        ),
        &Vector3d::new(0., 0., 0.),
    ));
    for _ in 0..200 {
        rbs.tick(0.01);
    }
    Ok(())
}
