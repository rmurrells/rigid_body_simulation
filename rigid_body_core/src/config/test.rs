use crate::{
    math::{
	moment_of_intertia,
	rotation_matrix,
	matrix::Matrix3x3,
	vector::Vector3d,
    },
    mesh::polyhedron_meshes,
    render::Color,
    RigidBody,
    RigidBodySimulation,
};
use std::f64::consts::PI;

pub fn immovable(rigid_body_simulation: &mut impl RigidBodySimulation) {
    rigid_body_simulation.camera_mut().position = Vector3d::new(0., 0., -10.);
    let mass_inv = 1.;
    let dim = Vector3d::new(5., 5., 5.);
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &dim,
	    mass_inv,
	    &Vector3d::new(0., -10., -10.),
	    &rotation_matrix::y(PI/4.),
	    &Vector3d::new(0., 2., 2.),
	    &Vector3d::new(2., 0., 0.),
	),
	Some((
	    polyhedron_meshes::cuboid(&dim),
	    Color::rgb(0, 255, 0),
	)),
    );

    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &Vector3d::new(10., 10., 10.),
	    0.,
	    &Vector3d::new(0., 0., 10.),
	    &Matrix3x3::identity(),
	    &Vector3d::new(0., 0., 0.),
	    &Vector3d::new(0., 0., 0.),
	),
	None,
    );
}

pub fn icosphere(rigid_body_simulation: &mut impl RigidBodySimulation, sd: u8) {
    let mesh = polyhedron_meshes::icosphere(5., sd);
    let mass_inv = 1.;
    let mi_inv = moment_of_intertia::solid_sphere(
	5., 1./mass_inv,
    ).inverse().expect("mi_inv");

    rigid_body_simulation.add_rigid_body(
	RigidBody::from_mesh(
	    &mesh,
	    mass_inv,
	    &mi_inv,
	    &Vector3d::new(4., -4., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::new(0., 0., 0.),
	    &Vector3d::new(1., 1., 1.),	
	),
 	Some((
	    mesh.clone(),
	    Color::rgb(0, 255, 0),
	)),
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::from_mesh(
	    &mesh,
	    mass_inv,
	    &mi_inv,
	    &Vector3d::new(-4., 4., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::new(0., 0., 0.),
	    &Vector3d::new(0., 1., 0.),	
	),
 	Some((
	    mesh,
	    Color::rgb(0, 255, 0),
	)),
    );
}

pub fn coincident(rigid_body_simulation: &mut impl RigidBodySimulation) {
    rigid_body_simulation.camera_mut().position = Vector3d::new(0., 0., -30.);
    let dim = Vector3d::new(5., 5., 5.);
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &dim,
	    1.,
	    &Vector3d::new(0., 0., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::new(0., 0., 0.),
	    &Vector3d::new(0., 0., 0.),
	),
	Some((
	    polyhedron_meshes::cuboid(&dim),
	    Color::rgb(255, 0, 0),
	)),
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &dim,
	    1.,
	    &Vector3d::new(0., 0., 0.),
	    &rotation_matrix::x(1.2).mult(&rotation_matrix::z(0.3)),
	    &Vector3d::new(0., 0., 0.),
	    &Vector3d::new(0., 0., 0.),
	),
	Some((
	    polyhedron_meshes::cuboid(&dim),
	    Color::rgb(0, 255, 0),
	)),
    );
}

pub fn bounding_box(rigid_body_simulation: &mut impl RigidBodySimulation) {
    rigid_body_simulation.camera_mut().position = Vector3d::new(0., 0., -30.);
    let radius = 2.25;
    let mass_inv = 1.;
    let mi_inv = moment_of_intertia::solid_sphere(
	radius, 1./mass_inv,
    ).inverse().expect("mi_inv");
    
    let mut dim = Vector3d::new(3., 3., 3.);
    for i in 0..3 {
	let x = i as f64*10.-10.;
	for j in 0..3 {
	    if j == 1 {continue;}
	    let y = j as f64*10.-10.;
	    for k in 0..3 {
		let z = k as f64*10.-10.;
		let color = Color::rgb(0, 255, 0);
		if j != 2 {
		    let mesh = polyhedron_meshes::icosphere(radius, 0);
		    rigid_body_simulation.add_rigid_body(
			RigidBody::from_mesh(
			    &mesh,
			    mass_inv,
			    &mi_inv,
			    &Vector3d::new(x, y, z),
			    &Matrix3x3::identity(),
			    &Vector3d::new(0., -4., 0.),
			    &Vector3d::new(0., 0., 0.),
			),
			Some((
			    mesh,
			    color,
			)),
		    );
		} else {
		    rigid_body_simulation.add_rigid_body(
			RigidBody::cuboid(
			    &dim,
			    mass_inv,
			    &Vector3d::new(x, y, z),
			    &Matrix3x3::identity(),
			    &Vector3d::new(0., -4., 0.),
			    &Vector3d::new(0., 0., 0.),
			),
			Some((
			    polyhedron_meshes::cuboid(&dim),
			    color,
			)),
		    );
		}
	    }
	}
    }
    
    dim = Vector3d::new(3., 3., 18.);
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &dim,
	    1./3.,
	    &Vector3d::new(-10., 0., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::new(0., -4., 0.),
	    &Vector3d::new(0., 0., 0.),
	),
	Some((
	    polyhedron_meshes::cuboid(&dim),
	    Color::rgb(0, 255, 0),
	)),
    );    
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &dim,
	    1./3.,
	    &Vector3d::new(0., 0., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::new(0., -4., 0.),
	    &Vector3d::new(0., 0., 0.),
	),
	Some((
	    polyhedron_meshes::cuboid(&dim),
	    Color::rgb(0, 255, 0),
	)),
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &dim,
	    1./3.,
	    &Vector3d::new(10., 0., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::new(0., -4., 0.),
	    &Vector3d::new(0., 0., 0.),
	),
	Some((
	    polyhedron_meshes::cuboid(&dim),
	    Color::rgb(0, 255, 0),
	)),
    );
    
    let dim = 29.;
    let dp1 = dim+1.;
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &Vector3d::new(dim, dim, dim),
	    0.,
	    &Vector3d::new(0., 0., -dp1),
	    &Matrix3x3::identity(),
	    &Vector3d::default(),
	    &Vector3d::default(),
	),
	None,
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &Vector3d::new(dim, dim, dim),
	    0.,
	    &Vector3d::new(0., 0., dp1),
	    &Matrix3x3::identity(),
	    &Vector3d::default(),
	    &Vector3d::default(),
	),
	None,
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &Vector3d::new(dim, dim, dim),
	    0.,
	    &Vector3d::new(0., -dp1, 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::default(),
	    &Vector3d::default(),
	),
	None,
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &Vector3d::new(dim, dim, dim),
	    0.,
	    &Vector3d::new(0., dp1, 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::default(),
	    &Vector3d::default(),
	),
	None,
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &Vector3d::new(dim, dim, dim),
	    0.,
	    &Vector3d::new(-dp1, 0., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::default(),
	    &Vector3d::default(),
	),
	None,
    );
    rigid_body_simulation.add_rigid_body(
	RigidBody::cuboid(
	    &Vector3d::new(dim, dim, dim),
	    0.,
	    &Vector3d::new(dp1, 0., 0.),
	    &Matrix3x3::identity(),
	    &Vector3d::default(),
	    &Vector3d::default(),
	),
	None,
    );
}
