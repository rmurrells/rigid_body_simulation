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
    n: usize, rigid_body_simulation: &mut impl RigidBodySimulationTrait,
) -> Result<(), String> {
    let bb_dim = 50.;
    let bb_dim = Vector3d::new(bb_dim, bb_dim, bb_dim);
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
    let icosahedron_mesh = polyhedron_meshes::regular_icosahedron(radius);
    let tetrahedron_mesh = polyhedron_meshes::regular_tetrahedron(radius);

    let mass = 1.;
    let mass_inv = 1./mass;
    let sphere_mi_inv = moment_of_inertia::solid_sphere(
	radius, mass,
    ).inverse().expect("sphere_mi_inv");
    let tetrahedron_mi_inv = moment_of_inertia::regular_tetrahedron(
	tetrahedron_mesh.vertices[0].dist(&tetrahedron_mesh.vertices[1]),
	mass,
    ).inverse().expect("tetrahedron_mi_inv");

    let get_pos = |axis: usize, index: usize, current: &mut f64| {
	let gap = (bb_dim[axis]-dim[axis]*n as f64)/(n+1) as f64;
	*current += gap+dim[axis]*if index == 0 {0.5} else{1.}
    };

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
		if j%3 == 0 {
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
			}
		    );
		} else if j%3 == 1 {
		    rigid_body_simulation.add_rigid_body(
			RigidBody::from_mesh(
			    &icosahedron_mesh,
			    mass_inv,
			    &sphere_mi_inv,
			    &Vector3d::new(x, y, z),
			    &Matrix3x3::identity(),
			    &Vector3d::new(0., -4., 0.),
			    &Vector3d::new(0., 0., 0.),
			)?,
			RenderOption::Mesh {
			    mesh: icosahedron_mesh.clone(),
			    color: color_increment.get(),
			},
		    );
		} else if j%3 == 2 {
		    rigid_body_simulation.add_rigid_body(
			RigidBody::from_mesh(
			    &tetrahedron_mesh,
			    mass_inv,
			    &tetrahedron_mi_inv,
			    &Vector3d::new(x, y, z),
			    &Matrix3x3::identity(),
			    &Vector3d::new(0., -4., 0.),
			    &Vector3d::new(0., 0., 0.),
			)?,
			RenderOption::Mesh {
			    mesh: tetrahedron_mesh.clone(),
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
    n: usize,
    colors: Vec<Color>,
    count: usize,
}

impl ColorIncrement {
    pub fn new(n: usize) -> Self {
	let mut colors = Vec::with_capacity(1021);
	let mut color = Color::rgb(255, 0, 0);
	let cap = colors.capacity();
	let cap4 = cap/4;
	for i in 0..cap {
	    colors.push(color);
	    if i < cap4 {color.g += 1;}
	    else if i < cap4*2 {color.r -= 1;}
	    else if i < cap4*3 {color.b += 1;}
	    else if i < cap {color.g -= 1;}
	    else {unreachable!();}
	}
	Self {
	    count: 0,
	    n,
	    colors,
	}
    }

    pub fn get(&mut self) -> Color {
	let index = self.count*self.colors.len()/self.n;
	self.count += 1;
	self.colors[index]
    }
}
