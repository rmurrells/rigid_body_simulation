pub mod test;

use crate::{
    CameraMode,
    math::{
	moment_of_inertia,
	matrix::Matrix3x3,
	vector::Vector3d,
    },
    mesh::polyhedron_meshes,
    render::{
	Color,
	RenderOption,
    },
    RigidBody,
    RigidBodySimulationTrait,
};

pub fn default(
    rigid_body_simulation: &mut impl RigidBodySimulationTrait,
) -> Result<(), String> {
    let bb_dim = Vector3d::new(40., 40., 40.);
    let bb_min = bb_dim.scale(-0.5);
    let bb_max = bb_dim.scale(0.5);
    rigid_body_simulation.set_bounding_box(Some((
	&bb_min, &bb_max,
	RenderOption::PolyhedronEdges{color: Color::rgb(255, 0, 0)},
    )));
    rigid_body_simulation.camera_mut().position = Vector3d::new(0., 0., -bb_dim[2]);
    rigid_body_simulation.camera_mover_mut().mode = CameraMode::Rel;
        
    let dim = Vector3d::new(3., 3., 3.);
    let radius = dim[0]/2.;
    let mass_inv = 1.;
    let sphere_mi_inv = moment_of_inertia::solid_sphere(
	radius, 1./mass_inv,
    ).inverse().expect("mi_inv");

    let n = 6;
    let get_pos = |axis: usize, index: usize, current: &mut f64| {
	let gap = (bb_dim[axis]-dim[axis]*n as f64)/(n+1) as f64;
	*current += gap+dim[axis]*if index == 0 {0.5} else{1.}
    };

    let icosphere_mesh = polyhedron_meshes::regular_icosahedron(radius);
    let mut color_increment = ColorIncrement::new(n*n*n);
    
    let mut x = bb_min[0];
    for i in 0..n {
	get_pos(0, i, &mut x);
	let mut y = bb_min[1];
	for j in 0..n {
	    get_pos(1, j, &mut y);
	    let mut z = bb_min[1];
	    for k in 0..n {
		get_pos(2, k, &mut z);
		if j%2 == 0 {
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
			    color: color_increment.get(),
			},
		    );
		} else {
		    rigid_body_simulation.add_rigid_body(
			RigidBody::from_mesh(
			    &icosphere_mesh,
			    mass_inv,
			    &sphere_mi_inv,
			    &Vector3d::new(x, y, z),
			    &Matrix3x3::identity(),
			    &Vector3d::new(0., -4., 0.),
			    &Vector3d::new(0., 0., 0.),
			)?,
			RenderOption::Mesh {
			    mesh: icosphere_mesh.clone(),
			    color: color_increment.get(),
			},
		    );
		}
	    }
	}
    }
    Ok(())
}

pub struct ColorIncrement {
    count: usize,
    n: usize,
}

impl ColorIncrement {
    pub fn new(n: usize) -> Self {
	Self {
	    count: 0,
	    n,
	}
    }

    pub fn get(&mut self) -> Color {
	let mut color = Color::rgb(255, 0, 0);
	for i in 0..self.count*1024/self.n {
	    match i/255 {
		0 => color.g += 1,
		1 => color.r -= 1,
		2 => color.b += 1,
		3 => color.g -= 1,
		_ => {
		    println!("Error, switch exceeds 3");
		    return color;
		}
	    }
	}
	self.count += 1;
	color
    }
}
