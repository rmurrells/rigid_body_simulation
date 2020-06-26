use super::{
    matrix::Matrix3x3,
    Quarternion,
    rotation_matrix,
};

#[test]
fn inverse() {
    let m = Matrix3x3::new(&[
	[2., 1., 1.],
	[3., 2., 1.],
	[2., 1., 2.],
    ]);
    println!("{}", m.inverse().unwrap());
    println!("{}", m.mult(&m.inverse().unwrap()));
    let m = Matrix3x3::new(&[
	[1., 2., -1.],
	[-2., 0., 1.],
	[1., -1., 0.],
    ]);
    println!("{}", m.inverse().unwrap());
    println!("{}", m.mult(&m.inverse().unwrap()));
}

#[test]
fn quarternion() {
    let m = Matrix3x3::identity();
    let q = Quarternion::from_matrix(&m);
    let m2 = q.to_matrix();
    println!("{}", m);
    println!("{}", m2);
    
    let m = rotation_matrix::x(1.2);
    let q = Quarternion::from_matrix(&m);
    let m2 = q.to_matrix();
    println!("{}", m);
    println!("{}", m2);

    let m = rotation_matrix::z(0.213213).mult(&m);
    let q = Quarternion::from_matrix(&m);
    let m2 = q.to_matrix();
    println!("{}", m);
    println!("{}", m2);    
}
