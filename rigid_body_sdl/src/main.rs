use rigid_body_sdl::{
    CameraMode,
    math::{
	matrix::Matrix3x3,
	moment_of_inertia,
	vector::Vector3d,
    },
    mesh::polyhedron_meshes,
    render::{
	Color,
	RenderOption,
    },
    rigid_body::RigidBody,
    RigidBodySimulationSDL,
    RigidBodySimulationTrait,
    StrResult,
};

fn main() -> StrResult<()> {
    let mut rigid_body_simulation = RigidBodySimulationSDL::new((800, 600))?;
    rigid_body_simulation.camera_mut().position = Vector3d::new(0., 0., -30.);
    rigid_body_simulation.camera_mover_mut().mode = CameraMode::Rel;
    let bbhd = 15.;
    rigid_body_simulation.set_bounding_box(Some((
	&Vector3d::new(-bbhd, -bbhd, -bbhd),
	&Vector3d::new(bbhd, bbhd, bbhd),
	RenderOption::PolyhedronEdges{color: Color::rgb(255, 0, 0)},
    )));

    let radius = 2.25;
    let mass_inv = 1.;
    let mi_inv = moment_of_inertia::solid_sphere(
	radius, 1./mass_inv,
    ).inverse().expect("mi_inv");
    
    let mut dim = Vector3d::new(3., 3., 3.);
    let color = Color::rgb(0, 255, 0);
    for i in 0..3 {
	let x = i as f64*10.-10.;
	for j in 0..3 {
	    if j == 1 {continue;}
	    let y = j as f64*10.-10.;
	    for k in 0..3 {
		let z = k as f64*10.-10.;
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
			RenderOption::Mesh{mesh, color},
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
			RenderOption::Mesh {
			    mesh: polyhedron_meshes::cuboid(&dim),
			    color,
			},
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
	RenderOption::Mesh {
	    mesh: polyhedron_meshes::cuboid(&dim),
	    color: Color::rgb(0, 255, 0),
	},
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
	RenderOption::Mesh {
	    mesh: polyhedron_meshes::cuboid(&dim),
	    color: Color::rgb(0, 255, 0),
	},
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
	RenderOption::Mesh {
	    mesh: polyhedron_meshes::cuboid(&dim),
	    color: Color::rgb(0, 255, 0),
	},
    );

    while rigid_body_simulation.tick()?{}
    Ok(())
}
