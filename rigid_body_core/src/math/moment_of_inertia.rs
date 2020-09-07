use super::{matrix::Matrix3x3, vector::Vector3d};

pub fn aligned_cuboid(dimensions: &Vector3d, mass: f64) -> Matrix3x3 {
    let x2 = dimensions[0] * dimensions[0];
    let y2 = dimensions[1] * dimensions[1];
    let z2 = dimensions[2] * dimensions[2];
    Matrix3x3::new(&[[y2 + z2, 0., 0.], [0., x2 + z2, 0.], [0., 0., x2 + y2]])
        .scale(mass / 12.)
}

pub fn solid_sphere(radius: f64, mass: f64) -> Matrix3x3 {
    Matrix3x3::identity().scale(mass * radius * radius * 2. / 5.)
}

pub fn regular_tetrahedron(side_length: f64, mass: f64) -> Matrix3x3 {
    Matrix3x3::identity().scale(mass * side_length * side_length / 20.)
}
