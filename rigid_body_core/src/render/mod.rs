mod camera;
mod draw_3d;
mod render_object_creator;
mod screen_buffer;

pub use camera::Camera;
pub use screen_buffer::{
    Color,
    PIXEL_FORMAT,
};
use crate::{
    math::{
	matrix_vector,
	matrix::Matrix3x3,
	polyhedron::Polyhedron,
	rotation_matrix,
	vector::Vector3d,
    },
    mesh::Mesh,
    SeparatingPlane,
    simulation::{
	rigid_body::RigidBody,
	Simulation,
    },
    utility::int_hash::IntMap,
    UID,
};
use draw_3d::Draw3d;
use std::f64::consts::PI;

type MeshMap = IntMap<UID, (Mesh, Color)>;

pub struct RendererCore {
    draw_3d: Draw3d,
    mesh_map: MeshMap,
}

impl RendererCore {
    pub fn new(window_size: (u32, u32)) -> Self {
	Self {
	    draw_3d: Draw3d::new(window_size),
	    mesh_map: MeshMap::default(),
	}
    }

    pub fn add_mesh(&mut self, uid: UID, mesh: Mesh, color: Color) {
	self.mesh_map.insert(uid, (mesh, color));
    }
    
    pub fn camera_mut(&mut self) -> &mut Camera {
	&mut self.draw_3d.camera
    }
    
    pub fn clear(&mut self, color: Color) {
	self.draw_3d.clear(color);
    }

    pub fn get_data(&self) -> &[u8] {
	self.draw_3d.get_data()
    }
    
    pub fn get_data_mut(&mut self) -> &mut [u8] {
	self.draw_3d.get_data_mut()
    }
    
    pub fn set_window_size(
	&mut self, window_size: (u32, u32),
    ) {
	self.draw_3d.set_window_size(window_size);
    }

    pub fn render_rigid_bodies(&mut self, rigid_bodies: &[RigidBody]) {
	for rigid_body in rigid_bodies {
	    self.draw_rigid_body(rigid_body, None);	    
	}
    }
    
    pub fn render_rigid_bodies_debug(
	&mut self, simulation: &Simulation,
    ) {
	for (i, rigid_body) in simulation.rigid_bodies.iter().enumerate() {
	    self.draw_rigid_body(
		rigid_body,
		Some(if simulation.collision_manager.is_colliding(i) {
		    (255, 0, 0)
		} else {
		    (0, 255, 0)
		}),
	    );

	    let bounding_box = rigid_body.bounding_box();
	    self.draw_aligned_cuboid(
		&bounding_box[0],
		&bounding_box[1],
		if simulation.collision_manager
		    .is_bounding_box_colliding(i)
		{
		    (0, 0, 255)
		} else {
		    (0, 255, 0)
		},
	    );
	}

	let len = simulation.rigid_bodies.len();
	for i in 1..len {
	    for j in 0..i {
		let collision_status = simulation
		    .collision_manager
		    .collision_table()
		    .get(i, j);
		if collision_status.bounding_box_collision() &&
		    !collision_status.colliding
		{
		    match collision_status.separating_plane {
			SeparatingPlane::Face{face_indices} => {
			    let polyhedron = simulation
				.rigid_bodies[face_indices.face_rigid_body]
				.polyhedron_world();
			    let edges = polyhedron.edges();
			    let vertices = polyhedron.vertices();
			    let separating_plane =
				&polyhedron.faces()[face_indices.face];
			    for edge_index in separating_plane.edge_indices() {
				let edge = &edges[*edge_index];
				self.draw_line(
				    &vertices[edge.start_index()],
				    &vertices[edge.end_index()],
				    (255, 0, 0),
				    true,
				);
			    }
			    let mut avg = Vector3d::default();
			    let vertex_indices =
				separating_plane.vertex_indices();
			    for vertex_index in vertex_indices {
				avg.add_assign(&vertices[*vertex_index]);
			    }
			    avg.scale_assign(1./vertex_indices.len() as f64);
			    self.draw_line(
				&avg, &avg.add(separating_plane.direction()),
				(255, 255, 255),
				true,
			    );
			}
			SeparatingPlane::Edge{edge_indices} => {
			    let plane_polyhedron = simulation
				.rigid_bodies[edge_indices.plane_rigid_body]
				.polyhedron_world();
			    let plane_vertices = plane_polyhedron.vertices();
			    let plane_edge =
				plane_polyhedron.edges()[edge_indices.plane_edge];
			    
			    let other_polyhedron = simulation
				.rigid_bodies[edge_indices.other_rigid_body]
				.polyhedron_world();
			    let other_vertices = other_polyhedron.vertices();
			    let other_edge =
				other_polyhedron.edges()[edge_indices.other_edge];

			    self.draw_edge_plane(
				&plane_vertices[plane_edge.start_index()],
				&plane_vertices[plane_edge.end_index()],
				&edge_indices
				    .plane_direction(&simulation.rigid_bodies)
				    .unwrap(),
				&other_vertices[other_edge.start_index()],
				&other_vertices[other_edge.end_index()],
			    );
			    
			    self.draw_line(
				&other_vertices[other_edge.start_index()],
				&other_vertices[other_edge.end_index()],
				(255, 0, 0),
				true,
			    );
			}
			SeparatingPlane::None => unreachable!(),
		    }
		}
	    }
	}
    }
    
    pub fn draw_aligned_cuboid(
	&mut self,
	min: &Vector3d,
	max: &Vector3d,
	color: Color,
    ) {
	self.draw_3d.draw_aligned_cuboid(min, max, color);
    }
    
    pub fn draw_line(
	&mut self,
	start: &Vector3d,
	end: &Vector3d,
	color: Color,
	in_front: bool
    ) {
	self.draw_3d.draw_line(start, end, color, in_front);
    }
    
    pub fn draw_mesh(
	&mut self,
	mesh: &Mesh,
	world_position: &Vector3d,
	world_orientation: &Matrix3x3,
	color: Color,
    ) {
	self.draw_3d.draw_mesh(mesh, world_position, world_orientation, color);
    }

    pub fn draw_polyhedron_wire_frame(
	&mut self,
	polyhedron: &Polyhedron,
	color: Color,
    ) {
	self.draw_3d.draw_polyhedron_wire_frame(
	    polyhedron, color,
	);
    }

    pub fn draw_position(
	&mut self,
	position: &Vector3d,
	color: Color,
    ) {
	self.draw_3d.draw_position(position, color);
    }

    pub fn draw_rigid_body_mesh_lines(
	&mut self,
	rigid_body: &RigidBody,
	color_opt: Option<Color>,
    ) {
	Self::draw_mesh_triangles_impl(
	    rigid_body, color_opt, &self.mesh_map, &mut self.draw_3d,
	);
    }
    
    pub fn draw_rigid_body(
	&mut self,
	rigid_body: &RigidBody,
	color_opt: Option<Color>,
    ) {
	Self::draw_rigid_body_impl(
	    rigid_body, color_opt, &self.mesh_map, &mut self.draw_3d,
	);
    }

    fn draw_edge_plane(
	&mut self,
	plane_edge_start: &Vector3d,
	plane_edge_end: &Vector3d,
	plane_direction: &Vector3d,
	other_edge_start: &Vector3d,
	other_edge_end: &Vector3d,
    ) {
	self.draw_line(
	    plane_edge_start, plane_edge_end, (255, 0, 0), true,
	);
	self.draw_line(
	    plane_edge_start,
	    &plane_edge_start.add(&other_edge_end.sub(other_edge_start)),
	    (255, 0, 0), true,
	);
	self.draw_line(
	    plane_edge_start, &plane_edge_start.add(plane_direction),
	    (255, 255, 255), true,
	);

	let mut start = plane_edge_end.sub(plane_edge_start);
	let mut end = matrix_vector::mult_3(
	    &rotation_matrix::general(plane_direction, PI/2.),
	    &start,
	);
	for _ in 0..4 {
	    self.draw_line(
		&start.add(plane_edge_start), &end.add(plane_edge_start),
		(255, 255, 255), true,
	    );
	    start = end;
	    end = matrix_vector::mult_3(
		&rotation_matrix::general(plane_direction, PI/2.),
		&start,
	    );
	}
    }
    
    fn draw_mesh_triangles_impl(
	rigid_body: &RigidBody,
	color_opt: Option<Color>,
	mesh_map: &MeshMap,
	draw_3d: &mut Draw3d,	
    ) {
	if let Some((mesh, mesh_color)) = mesh_map.get(&rigid_body.uid()) {
	    draw_3d.draw_mesh_lines(
		mesh,
		&rigid_body.position,
		rigid_body.rotation(),
		*match &color_opt {
		    Some(color) => color,
		    None => mesh_color,
		},
		true,
	    );
	}
    }
    
    fn draw_rigid_body_impl(
	rigid_body: &RigidBody,
	color_opt: Option<Color>,
	mesh_map: &MeshMap,
	draw_3d: &mut Draw3d,
    ) {
	match mesh_map.get(&rigid_body.uid()) {
	    Some((mesh, mesh_color)) => draw_3d.draw_mesh(
		mesh,
		&rigid_body.position,
		rigid_body.rotation(),
		*match &color_opt {
		    Some(color) => color,
		    None => mesh_color,
		}
	    ),
	    None => draw_3d.draw_polyhedron_wire_frame(
		rigid_body.polyhedron_world(),
		*match &color_opt {
		    Some(color) => color,
		    None => &(255, 0, 255),
		}
	    ),
	}
    }    
}
