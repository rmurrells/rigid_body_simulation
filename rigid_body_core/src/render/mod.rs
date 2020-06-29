mod camera;
mod draw_3d;
mod render_object_creator;
mod screen_buffer;

pub use camera::Camera;
pub use draw_3d::Draw3dTrait;
pub use screen_buffer::{
    Color,
    PIXEL_FORMAT,
    ScreenBufferTrait,
};
use crate::{
    math::{
	matrix_vector,
	polyhedron::Polyhedron,
	rotation_matrix,
	vector::Vector3d,
    },
    mesh::Mesh,
    simulation::{
	rigid_body::RigidBody,
	Simulation,
    },
    SeparatingPlane,
    utility::int_hash::IntMap,
    UID,
};
use draw_3d::{
    Draw3d,
    Draw3dAccess,
};
use screen_buffer::{
    ScreenBuffer,
    ScreenBufferAccess,
};
use std::f64::consts::PI;

type MeshMap = IntMap<UID, RenderOption>;

pub enum RenderOption {
    Mesh{mesh: Mesh, color: Color},
    PolyhedronEdges{color: Color},
    FaceEdges{face_indices: Vec<usize>, color: Color},
    None,
}

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

    pub fn add_mesh(&mut self, uid: UID, render_option: RenderOption) {
	self.mesh_map.insert(uid, render_option);
    }
    
    pub fn render_rigid_bodies(&mut self, rigid_bodies: &[RigidBody]) {
	for rigid_body in rigid_bodies {
	    self.draw_rigid_body(rigid_body, &None);	    
	}
    }
    
    pub fn render_rigid_bodies_debug(
	&mut self, simulation: &Simulation,
    ) {
	for (i, rigid_body) in simulation.rigid_bodies.iter().enumerate() {
	    self.draw_rigid_body(
		rigid_body,
		&Some(if simulation.collision_manager.is_colliding(i) {
		    Color::rgb(255, 0, 0)
		} else {
		    Color::rgb(0, 255, 0)
		}),
	    );
	    let bounding_box = rigid_body.bounding_box();
	    self.draw_aligned_cuboid(
		&bounding_box[0],
		&bounding_box[1],
		if simulation.collision_manager
		    .is_bounding_box_colliding(i)
		{
		    Color::rgb(0, 0, 255)
		} else {
		    Color::rgb(0, 255, 0)
		},
	    );
	}
	for i in 1..simulation.rigid_bodies.len() {
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
			    Self::draw_face_edges(
				polyhedron,
				face_indices.face,
				Color::rgb(255, 0, 0),
				true,
				&mut self.draw_3d,
			    );
			    let mut avg = Vector3d::default();
			    let vertices = polyhedron.vertices();
			    let separating_face =
				&polyhedron.faces()[face_indices.face];
			    let vertex_indices =
				separating_face.vertex_indices();
			    for vertex_index in vertex_indices {
				avg.add_assign(&vertices[*vertex_index]);
			    }
			    avg.scale_assign(1./vertex_indices.len() as f64);
			    self.draw_line(
				&avg, &avg.add(separating_face.direction()),
				Color::rgb(255, 255, 255),
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
				Color::rgb(255, 0, 0),
				true,
			    );
			}
			SeparatingPlane::None => unreachable!(),
		    }
		}
	    }
	}
    }

    pub fn draw_rigid_body_mesh_lines(
	&mut self,
	rigid_body: &RigidBody,
	color_opt: &Option<Color>,
    ) {
	Self::draw_mesh_triangles_impl(
	    rigid_body, color_opt, &self.mesh_map, &mut self.draw_3d,
	);
    }
    
    pub fn draw_rigid_body(
	&mut self,
	rigid_body: &RigidBody,
	color_opt: &Option<Color>,
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
	    plane_edge_start, plane_edge_end, Color::rgb(255, 0, 0), true,
	);
	self.draw_line(
	    plane_edge_start,
	    &plane_edge_start.add(&other_edge_end.sub(other_edge_start)),
	    Color::rgb(255, 0, 0), true,
	);
	self.draw_line(
	    plane_edge_start, &plane_edge_start.add(plane_direction),
	    Color::rgb(255, 255, 255), true,
	);

	let mut start = plane_edge_end.sub(plane_edge_start);
	let mut end = matrix_vector::mult_3(
	    &rotation_matrix::general(plane_direction, PI/2.),
	    &start,
	);
	for _ in 0..4 {
	    self.draw_line(
		&start.add(plane_edge_start), &end.add(plane_edge_start),
		Color::rgb(255, 255, 255), true,
	    );
	    start = end;
	    end = matrix_vector::mult_3(
		&rotation_matrix::general(plane_direction, PI/2.),
		&start,
	    );
	}
    }

    fn draw_default(
	rigid_body: &RigidBody, color_opt: &Option<Color>, draw_3d: &mut Draw3d,
    ) {
	draw_3d.draw_polyhedron_edges(
	    rigid_body.polyhedron_world(),
	    match color_opt {
		Some(opt_color) => *opt_color,
		None => Color::rgb(255, 0, 255),
	    }
	)
    }

    fn draw_face_edges(
	polyhedron: &Polyhedron,
	face_index: usize,
	color: Color,
	in_front: bool,
	draw_3d: &mut Draw3d,
    ) {
	let vertices = polyhedron.vertices();
	let edges = polyhedron.edges();
	for edge_index in polyhedron.faces()[face_index].edge_indices() {
	    let edge = &edges[*edge_index];
	    draw_3d.draw_line(
		&vertices[edge.start_index()],
		&vertices[edge.end_index()],
		color,
		in_front,
	    );
	}
    }
    
    fn draw_mesh_triangles_impl(
	rigid_body: &RigidBody,
	color_opt: &Option<Color>,
	mesh_map: &MeshMap,
	draw_3d: &mut Draw3d,
    ) {
	if let Some(render_option) = mesh_map.get(&rigid_body.uid()) {
	    if let RenderOption::Mesh{mesh, color} = render_option {
		draw_3d.draw_mesh_lines(
		    mesh,
		    &rigid_body.position,
		    rigid_body.rotation(),
		    *match color_opt {
			Some(opt_color) => opt_color,
			None => color,
		    },
		    true,
		);
	    }
	}
    }
    
    fn draw_rigid_body_impl(
	rigid_body: &RigidBody,
	color_opt: &Option<Color>,
	mesh_map: &MeshMap,
	draw_3d: &mut Draw3d,
    ) {
	match mesh_map.get(&rigid_body.uid()) {
	    Some(render_option) => match render_option {
		RenderOption::Mesh{mesh, color} => {
		    draw_3d.draw_mesh(
			mesh,
			&rigid_body.position,
			rigid_body.rotation(),
			*match color_opt {
			    Some(opt_color) => opt_color,
			    None => color,
			}
		    );
		}
		RenderOption::PolyhedronEdges{color} => {
		    draw_3d.draw_polyhedron_edges(
			rigid_body.polyhedron_world(),
			*match color_opt {
			    Some(opt_color) => opt_color,
			    None => color,
			}
		    );   
		}
		RenderOption::FaceEdges{face_indices, color} => {
		    for face_index in face_indices {
			Self::draw_face_edges(
			    rigid_body.polyhedron_world(),
			    *face_index,
 			    *match color_opt {
				Some(opt_color) => opt_color,
				None => color,
			    },
			    false,
			    draw_3d,
			);
		    }
		}
		RenderOption::None =>
		    Self::draw_default(rigid_body, color_opt, draw_3d),
	    },
	    None => Self::draw_default(rigid_body, color_opt, draw_3d),
	}
    }    
}

impl Draw3dAccess for RendererCore {
    fn draw_3d_access(&self) -> &Draw3d {
	&self.draw_3d
    }
    fn draw_3d_access_mut(&mut self) -> &mut Draw3d {
	&mut self.draw_3d
    }
}
impl Draw3dTrait for RendererCore {}

impl ScreenBufferAccess for RendererCore {
    fn screen_buffer_access(&self) -> &ScreenBuffer {
	self.draw_3d.screen_buffer_access()
    }
    fn screen_buffer_access_mut(&mut self) -> &mut ScreenBuffer {
	self.draw_3d.screen_buffer_access_mut()
    }
}
impl ScreenBufferTrait for RendererCore {}
