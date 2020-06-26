#![allow(dead_code)]
use super::camera::Camera;
use crate::{
    math::{
	geometry::{
	    self,
	    FiniteLine3d,
	    Plane,
	},
	matrix::{
	    Matrix3x3,
	    Matrix4x4,
	},
	matrix_vector,
	triangle::{
	    Triangle3d,
	    Triangle4d,
	},
	vector::{
	    Vector3d,
	    Vector4d,
	},
    },
    mesh::MeshTriangle,
};
use std::f64::EPSILON;

#[derive(Clone, Copy)]
pub struct RenderObjectCreator {
    window_hsize: (f64, f64),
    near: f64,
    far: f64,
    fov: f64,
    projection_matrix: Matrix4x4,
    near_plane: Plane,
    clip_planes: [Plane; 4],
}

impl RenderObjectCreator {
    pub fn new(
	window_size: (u32, u32),
	near: f64,
	far: f64,
	fov: f64,
    ) -> Self {
	let window_hsize = (
	    f64::from(window_size.0)/2.,
	    f64::from(window_size.1)/2.,
	);
	Self{
	    window_hsize,
	    near,
	    far,
	    fov,
	    projection_matrix: Self::get_projection_matrix(
		&window_hsize, near, far, fov,
	    ),
	    near_plane: Plane{
		pos: Vector3d::new(0., 0., near),
		dir: Vector3d::new(0., 0., 1.),
	    },
	    clip_planes: [
		Plane{
		    pos: Vector3d::new(-1., 0., 0.),
		    dir: Vector3d::new(1., 0., 0.),
		},
		Plane{
		    pos: Vector3d::new(1., 0., 0.),
		    dir: Vector3d::new(-1., 0., 0.),
		},
		Plane{
		    pos: Vector3d::new(0., -1., 0.),
		    dir: Vector3d::new(0., 1., 0.),
		},
		Plane{
		    pos: Vector3d::new(0., 1., 0.),
		    dir: Vector3d::new(0., -1., 0.),
		},
	    ]
	}
    }

    pub fn set_window_size(&mut self, window_size: (u32, u32)) {
	self.window_hsize = (
	    f64::from(window_size.0)/2.,
	    f64::from(window_size.1)/2.,
	);
	self.projection_matrix = Self::get_projection_matrix(
	    &self.window_hsize, self.near, self.far, self.fov,
	);
    }
    
    pub fn get_fov(&self) -> f64 {
	self.fov
    }
    
    pub fn get_far(&self) -> f64 {
	self.far
    }

    pub fn get_near(&self) -> f64 {
	self.near
    }

    pub fn get_window_line(
	&self,
	start: &Vector3d,
	end: &Vector3d,
	camera: &Camera,
    ) -> Option<RenderLine> {
	let mut ret = RenderLine::from_vertices(
	    &camera.view(start),
	    &camera.view(end),
	);
	Self::clip_line_on_plane(&self.near_plane, &mut ret.finite_line_3d)?;
	ret.finite_line_3d.start = Self::dehomogenized_pos(
	    &self.projected_pos(&ret.finite_line_3d.start),
	);
	ret.finite_line_3d.end = Self::dehomogenized_pos(
	    &self.projected_pos(&ret.finite_line_3d.end),
	);
	for plane in &self.clip_planes {
	    Self::clip_line_on_plane(plane, &mut ret.finite_line_3d)?;
	}
	self.map_pos_to_window(&mut ret.finite_line_3d.start);
	self.map_pos_to_window(&mut ret.finite_line_3d.end);
	
	Some(ret)
    }

    pub fn get_window_pos(
	&self,
	pos: &Vector3d,
	camera: &Camera,
    ) -> Option<Vector3d> {
	let camera_pos = camera.view(pos);
	Self::clip_pos_on_plane(&self.near_plane, &camera_pos)?;
	let mut dpos =
	    Self::dehomogenized_pos(&self.projected_pos(&camera_pos));
	for plane in &self.clip_planes {
	    Self::clip_pos_on_plane(plane, &dpos)?;
	}
	self.map_pos_to_window(&mut dpos);
	Some(dpos)
    }
    
    pub fn get_window_triangles(
	&self,
	mesh_triangles: &[MeshTriangle],
	world_position: &Vector3d,
	world_orientation: &Matrix3x3,
	camera: &Camera,
    ) -> Vec<RenderTriangle> {
	let mut ret = Vec::with_capacity(mesh_triangles.len());
	for mesh_triangle in mesh_triangles {
	    let world_mesh_triangle = {
		let vertices = &mesh_triangle.triangle_3d.vertices;
		MeshTriangle::norm_from_vertices(
		    &matrix_vector::mult_3(
			&world_orientation, &vertices[0],
		    ).add(&world_position),
		    &matrix_vector::mult_3(
			&world_orientation, &vertices[1],
		    ).add(&world_position),
		    &matrix_vector::mult_3(
			&world_orientation, &vertices[2],
		    ).add(&world_position),
		)
	    };

	    if world_mesh_triangle.normal.dot(&camera.position.sub(
		&world_mesh_triangle.triangle_3d.vertices[0],
	    )) < 0. {continue;}
	    let camera_triangle = Self::camera_triangle(
		camera, &world_mesh_triangle.triangle_3d,
	    );

	    let mut add_render_triangle = |render_triangle: &RenderTriangle| {
		let projected_triangle =
		    self.projected_triangle(&render_triangle.triangle_3d);
		let dehomogenized_triangle =
		    Self::dehomogenized_triangle(&projected_triangle);
		ret.push(RenderTriangle::new(
		    &dehomogenized_triangle,
		    world_mesh_triangle.normal.dot(camera.dir()),
		));
	    };
	    
	    let light_value = world_mesh_triangle.normal.dot(camera.dir());
	    match self.clip_triangle_on_plane(
		&RenderTriangle::new(
		    &camera_triangle,
		    light_value,
		),
		&self.near_plane,
	    ) {
		ClipTriangleOnPlane::Inside => add_render_triangle(
		    &RenderTriangle::new(
			&camera_triangle,
			light_value,
		    ),
		),
		ClipTriangleOnPlane::PartOne(new) => add_render_triangle(
		    &new,
		),
		ClipTriangleOnPlane::PartTwo(new_1, new_2) => {
		    add_render_triangle(&new_1);
		    add_render_triangle(&new_2);
		}
		ClipTriangleOnPlane::Outside => (),
	    }	    
	}

	self.clip_triangles(&mut ret);
	self.map_triangles_to_window(&mut ret);
	ret
    }    

    fn camera_triangle(
	camera: &Camera,
	triangle: &Triangle3d,
    ) -> Triangle3d {
	Triangle3d::new(
	    &camera.view(&triangle.vertices[0]),
	    &camera.view(&triangle.vertices[1]),
	    &camera.view(&triangle.vertices[2]),
	)
    }

    fn clip_line_on_plane(
	plane: &Plane,
	line: &mut FiniteLine3d,
    ) -> Option<()> {
	let ds = geometry::pos_plane_signed_dist(&line.start, plane);
	let de = geometry::pos_plane_signed_dist(&line.end, plane);
	if ds < 0. && de < 0. {return None;}
	if ds >= 0. && de >= 0. {return Some(());}
	let intercept = Self::get_intercept(
	    plane,
	    &line.start,
	    &line.end,
	);
	if ds >= 0. {
	    line.end = intercept;
	} else {
	    line.start = intercept;
	}
	Some(())
    }

    fn clip_pos_on_plane(
	plane: &Plane,
	pos: &Vector3d,
    ) -> Option<()> {
	if geometry::pos_plane_signed_dist(pos, plane) < 0. {return None;}
	Some(())
    }
    
    fn clip_triangles(
	&self,
	triangles: &mut Vec<RenderTriangle>,
    ) {
	let mut triangles_to_add = Vec::new();
	for plane_index in 0..self.clip_planes.len() {
	    let mut size = triangles.len();
	    let mut i = 0;
	    while i < size {
		let triangle = &mut triangles[i];
		match self.clip_triangle_on_plane(
		    &triangle,
		    &self.clip_planes[plane_index]
		) {
		    ClipTriangleOnPlane::Inside => (),
		    ClipTriangleOnPlane::PartOne(new) => {
			*triangle = new;
		    }
		    ClipTriangleOnPlane::PartTwo(new_1, new_2) => {
			*triangle = new_1;
			triangles_to_add.push(new_2);
		    }
		    ClipTriangleOnPlane::Outside => {
			size -= 1;
			triangles.swap(i, size);
			continue;
		    },
		}
		i += 1;
	    }
	    triangles.truncate(size);
	    triangles.append(&mut triangles_to_add);
	}
    }

    fn clip_triangle_on_plane(
	&self,
	render_triangle: &RenderTriangle,
	plane: &Plane,
    ) -> ClipTriangleOnPlane {
	let triangle_vertices = &render_triangle.triangle_3d.vertices;
	let mut inside = [0usize; 3];
	let mut inside_count = 0usize;
	let mut outside = [0usize; 3];
	let mut outside_count = 0usize;
	for (i, vertex) in triangle_vertices.iter().enumerate().take(3) {
	    if geometry::pos_plane_signed_dist(&vertex, plane) < 0. {
		outside[outside_count] = i;
		outside_count += 1;
	    } else {
		inside[inside_count] = i;
		inside_count += 1;
	    }
	}
	match inside_count {
	    0 => ClipTriangleOnPlane::Outside,
	    1 => {
		let inside_vert = &triangle_vertices[inside[0]];
		let intercept_1 = Self::get_intercept(
		    plane,
		    &inside_vert,
		    &triangle_vertices[outside[0]],
		);
		let intercept_2 = Self::get_intercept(
		    plane,
		    &inside_vert,
		    &triangle_vertices[outside[1]],
		);

		ClipTriangleOnPlane::PartOne(
		    RenderTriangle::from_vertices(
			inside_vert,
			&intercept_1,
			&intercept_2,
			render_triangle.light_value,
		    ),
		)
	    }
	    2 => {
		let inside_vert_1 = &triangle_vertices[inside[0]];
		let inside_vert_2 = &triangle_vertices[inside[1]];
		let outside_vert = &triangle_vertices[outside[0]];
		
		let intercept_1  = Self::get_intercept(
		    plane,
		    outside_vert,
		    inside_vert_1,
		);
		let intercept_2 = Self::get_intercept(
		    plane,
		    outside_vert,
		    inside_vert_2,
		);
		
		ClipTriangleOnPlane::PartTwo(
		    RenderTriangle::from_vertices(
			&inside_vert_1,
			&inside_vert_2,
			&intercept_1,
			render_triangle.light_value,
		    ),
		    RenderTriangle::from_vertices(
			&inside_vert_2,
			&intercept_1,
			&intercept_2,
			render_triangle.light_value,
		    ),
		    
		)
	    },
	    3 => ClipTriangleOnPlane::Inside,
	    _ => unreachable!(),
	}
    }

    fn dehomogenized_pos(pos: &Vector4d) -> Vector3d {
	let mut div = pos[3];
	if pos[3] < EPSILON {
	    div = 1.
	}
        Vector3d::new(
	    pos[0]/div,
	    pos[1]/div,
	    pos[2]/div,
	)
    }

    fn dehomogenized_triangle(triangle: &Triangle4d) -> Triangle3d {	
	Triangle3d::new(
	    &Self::dehomogenized_pos(&triangle.vertices[0]),
	    &Self::dehomogenized_pos(&triangle.vertices[1]),
	    &Self::dehomogenized_pos(&triangle.vertices[2]),
	)
    }

    fn get_intercept(
	plane: &Plane,
	inside: &Vector3d,
	outside: &Vector3d,
    ) -> Vector3d {
	if let Some(intercept) = &geometry::plane_finite_line_intersection(
	    plane, inside, outside,
	) {
	    *intercept
	} else {
	    *outside
	}
    }

    fn get_projection_matrix(
	window_hsize: &(f64, f64),
	near: f64,
	far: f64,
	fov: f64,
    ) -> Matrix4x4 {
        let mut projection_matrix = Matrix4x4::default();
        let cot = 1./(fov/2.).tan();
        let length = far-near;
        projection_matrix[0][0] = cot*window_hsize.1/window_hsize.0;
        projection_matrix[1][1] = cot;
	projection_matrix[2][2] = far/length;
        projection_matrix[2][3] = -far*near/length;
        projection_matrix[3][2] = 1.;
	projection_matrix
    }
    
    fn projected_pos(&self, pos: &Vector3d) -> Vector4d {
	let mut ret = matrix_vector::mult_4(
            &self.projection_matrix,
            &Vector4d::new(pos[0], pos[1], pos[2], 1.),
	);
	ret[0] *= -1.;
	ret[1] *= -1.;
	ret
    }

    fn projected_triangle(&self, triangle: &Triangle3d) -> Triangle4d {
	Triangle4d::new(
	    &self.projected_pos(&triangle.vertices[0]),
	    &self.projected_pos(&triangle.vertices[1]),
	    &self.projected_pos(&triangle.vertices[2]),
	)
    }

    fn map_triangles_to_window(
	&self,
	triangles: &mut [RenderTriangle],
    ) {
	for triangle in triangles.iter_mut() {
	    for vertex in triangle.triangle_3d.vertices.iter_mut() {
		self.map_pos_to_window(vertex);
	    }
	}
    }

    fn map_pos_to_window(
	&self,
	pos: &mut Vector3d,
    ) {
	pos[0] = (pos[0]+1.)*self.window_hsize.0;
	pos[1] = (pos[1]+1.)*self.window_hsize.1;
	pos[2] = -pos[2]+1.;
    }
}

pub struct RenderLine {
    pub finite_line_3d: FiniteLine3d,
}

impl RenderLine {
    fn from_vertices(
	vertex_1: &Vector3d,
	vertex_2: &Vector3d,
    ) -> Self {
	Self {
	    finite_line_3d: FiniteLine3d {
		start: *vertex_1,
		end: *vertex_2,
	    }
	}
    }
}

pub struct RenderTriangle {
    pub triangle_3d: Triangle3d,
    pub light_value: f64,
}

impl RenderTriangle {
    fn new(
	triangle_3d: &Triangle3d,
	light_value: f64,
    ) -> Self {
	Self {
	    triangle_3d: *triangle_3d,
	    light_value,
	}
    }

    fn from_vertices(
	vertex_1: &Vector3d,
	vertex_2: &Vector3d,
	vertex_3: &Vector3d,
	light_value: f64,
    ) -> Self {
	Self {
	    triangle_3d: Triangle3d::new(
		vertex_1, vertex_2, vertex_3,
	    ),
	    light_value,
	}
    }
}

enum ClipTriangleOnPlane {
    Inside,
    PartOne(RenderTriangle),
    PartTwo(RenderTriangle, RenderTriangle),
    Outside,
}
