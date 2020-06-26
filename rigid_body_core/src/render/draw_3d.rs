use crate::{
    math::{
	matrix::Matrix3x3,
	polyhedron::Polyhedron,
	vector::Vector3d,
    },
    mesh::Mesh,
};
use super::{
    camera::Camera,
    render_object_creator::RenderObjectCreator,
    screen_buffer::{
	ScreenBuffer,
	Color,
    },
};
use std::f64::consts::PI;

pub struct Draw3d {
    pub camera: Camera,
    screen_buffer: ScreenBuffer,
    render_object_creator: RenderObjectCreator,
}

impl Draw3d {
    pub fn new(window_size: (u32, u32)) -> Self {
	Self {
            camera: Camera::default(),
            screen_buffer: ScreenBuffer::new(window_size),
            render_object_creator: RenderObjectCreator::new(
                window_size,
                0.1,
                20000.,
                PI/2.,
            ),
	}
    }

    pub fn clear(&mut self, color: Color) {
	self.screen_buffer.clear(color);
    }

    pub fn get_data(&self) -> &[u8] {
	self.screen_buffer.get_data()
    }
    
    pub fn get_data_mut(&mut self) -> &mut [u8] {
	self.screen_buffer.get_data_mut()
    }
    
    pub fn get_simple_light_color(
	color: Color,
	light: f64,
    ) -> Color {
	let light = -(light-1.)/2.;
	(
	    (color.0 as f64 * light) as u8,
	    (color.1 as f64 * light) as u8,
	    (color.2 as f64 * light) as u8,
	)
    }

    pub fn set_window_size(&mut self, window_size: (u32, u32)) {
	self.render_object_creator.set_window_size(window_size);
	self.screen_buffer.resize(window_size);
    }
    
    pub fn draw_aligned_cuboid(
	&mut self,
	min: &Vector3d,
	max: &Vector3d,
	color: Color,
    ) {
	self.draw_line(min, &Vector3d::new(max[0], min[1], min[2]), color, false);
	self.draw_line(&Vector3d::new(max[0], min[1], min[2]), &Vector3d::new(max[0], max[1], min[2]), color, false);
	self.draw_line(&Vector3d::new(max[0], max[1], min[2]), &Vector3d::new(min[0], max[1], min[2]), color, false);
	self.draw_line(&Vector3d::new(min[0], max[1], min[2]), min, color, false);

	self.draw_line(&Vector3d::new(min[0], min[1], max[2]), &Vector3d::new(max[0], min[1], max[2]), color, false);
	self.draw_line(&Vector3d::new(max[0], min[1], max[2]), max, color, false);
	self.draw_line(max, &Vector3d::new(min[0], max[1], max[2]), color, false);
	self.draw_line(&Vector3d::new(min[0], max[1], max[2]), &Vector3d::new(min[0], min[1], max[2]), color, false);

	self.draw_line(min, &Vector3d::new(min[0], min[1], max[2]), color, false);
	self.draw_line(&Vector3d::new(max[0], min[1], min[2]), &Vector3d::new(max[0], min[1], max[2]), color, false);
	self.draw_line(&Vector3d::new(max[0], max[1], min[2]), max, color, false);
	self.draw_line(&Vector3d::new(min[0], max[1], min[2]), &Vector3d::new(min[0], max[1], max[2]), color, false);
    }
    
    pub fn draw_line(
	&mut self,
	start: &Vector3d,
	end: &Vector3d,
	color: Color,
	in_front: bool
    ) {
	if let Some(window_line) = &self.render_object_creator.get_window_line(
	    start, end, &self.camera,
	) {
	    self.screen_buffer.draw_clipped_line(
		&window_line.finite_line_3d.start,
		&window_line.finite_line_3d.end,
		color, in_front,
	    );
	}
    }
    
    pub fn draw_mesh(
	&mut self,
	mesh: &Mesh,
	world_position: &Vector3d,
	world_orientation: &Matrix3x3,
	color: Color,
    ) {
	for window_triangle in self.render_object_creator.get_window_triangles(
	    &mesh.mesh_triangles,
	    world_position,
	    world_orientation,
	    &self.camera,
	) {
	    let color = Self::get_simple_light_color(
		color, window_triangle.light_value,
	    );
	    self.screen_buffer.fill_triangle(
		&window_triangle.triangle_3d, color,
	    );
	}
    }

    pub fn draw_mesh_lines(
	&mut self,
	mesh: &Mesh,
	world_position: &Vector3d,
	world_orientation: &Matrix3x3,
	color: Color,
	in_front: bool,
    ) {
	for window_triangle in self.render_object_creator.get_window_triangles(
	    &mesh.mesh_triangles,
	    world_position,
	    world_orientation,
	    &self.camera,
	) {
	    self.screen_buffer.draw_triangle_lines(
		&window_triangle.triangle_3d, color, in_front,
	    );
	}
    }
    
    pub fn draw_polyhedron_wire_frame(
	&mut self,
	polyhedron: &Polyhedron,
	color: Color,
    ) {
	let vertices = polyhedron.vertices();
	for edge in polyhedron.edges() {
	    self.draw_line(
		&vertices[edge.start_index()],
		&vertices[edge.end_index()],
		color,
		false,
	    );
	}
    }

    pub fn draw_position(
	&mut self,
	position: &Vector3d,
	color: Color,
    ) {
	if let Some(window_position) =
	    &self.render_object_creator.get_window_pos(
		position, &self.camera,
	    )
	{
	    self.screen_buffer.draw_position(window_position, color);
	}
    }
}
