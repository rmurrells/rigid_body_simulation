use super::{
    geometry,
    vector::Vector3d,
};

#[derive(Clone)]
pub struct Polyhedron {
    faces: Vec<Face>,
    edges: Vec<Edge>,
    vertices: Vec<Vector3d>,
}

impl Polyhedron {
    pub fn new(
	face_vertex_indices: Vec<Vec<usize>>,
	edges: Vec<Edge>,
	vertices: Vec<Vector3d>,
    ) -> Result<Self, String> {
	let mut inside = Vector3d::default();
	for vertex in &vertices {inside.add_assign(vertex);}
	inside.scale_assign(1./vertices.len() as f64);
	let mut faces = Vec::with_capacity(face_vertex_indices.len());
	for indices in face_vertex_indices {
	    faces.push(Face::new(indices,  &vertices, &inside, &edges)?);
	}
	Ok(Self {
	    faces,
	    edges,
	    vertices,
	})
    }

    pub fn cuboid(
	dimensions: &Vector3d,	
    ) -> Self {
	let hx = dimensions[0]/2.;
	let hy = dimensions[1]/2.;
	let hz = dimensions[2]/2.;
	let vertices = vec![
	    Vector3d::new(-hx, -hy, -hz), Vector3d::new(hx, -hy, -hz),
	    Vector3d::new(hx, hy, -hz), Vector3d::new(-hx, hy, -hz),
	    Vector3d::new(-hx, -hy, hz), Vector3d::new(hx, -hy, hz),
	    Vector3d::new(hx, hy, hz), Vector3d::new(-hx, hy, hz),
	];
	Self::new(
	    vec![
		vec![0, 3, 4, 7],	
                vec![1, 2, 5, 6],
		vec![0, 1, 4, 5],
                vec![2, 3, 6, 7],
		vec![0, 1, 2, 3],		
		vec![4, 5, 6, 7],
	    ],
	    vec![
		Edge::new(0, 1, &vertices),
		Edge::new(1, 2, &vertices),
		Edge::new(2, 3, &vertices),
		Edge::new(3, 0, &vertices),
		
		Edge::new(0, 4, &vertices),
		Edge::new(1, 5, &vertices),
		Edge::new(2, 6, &vertices),
		Edge::new(3, 7, &vertices),
		
		Edge::new(4, 5, &vertices),
		Edge::new(5, 6, &vertices),
		Edge::new(6, 7, &vertices),
		Edge::new(7, 4, &vertices),
	    ],
	    vertices,
	).expect("Polyhedron - cuboid")
    }

    pub fn update(&mut self) {
	for edge in &mut self.edges {
	    edge.update(&self.vertices);
	}
	for face in &mut self.faces {
	    face.update(&self.edges);
	}
    }

    pub fn edges(&self) -> &[Edge] {
	&self.edges
    }	

    pub fn faces(&self) -> &[Face] {
	&self.faces
    }

    pub fn get_refs(&self) -> (&[Vector3d], &[Edge], &[Face]) {
	(self.vertices(), self.edges(), self.faces())
    }

    pub fn vertices(&self) -> &[Vector3d] {
	&self.vertices
    }
    
    pub fn vertices_mut(&mut self) -> &mut [Vector3d] {
	&mut self.vertices
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    start_index: usize,
    end_index: usize,
    direction: Vector3d,
}

impl Edge {
    pub fn new(
	start_index: usize, end_index: usize, vertices: &[Vector3d],
    ) -> Self {
	let mut ret = Self {
	    start_index,
	    end_index,
	    direction: Vector3d::default(),
	};
	ret.update(vertices);
	ret
    }

    pub fn direction(&self) -> &Vector3d {
	&self.direction
    }

    pub fn start_index(&self) -> usize {
	self.start_index
    }

    pub fn end_index(&self) -> usize {
	self.end_index
    }
    
    fn update(&mut self, vertices: &[Vector3d]) {
	self.direction = vertices[self.end_index]
	    .sub(&vertices[self.start_index])
	    .normal();
    }
}

#[derive(Clone)]
pub struct Face {
    vertex_indices: Vec<usize>,
    connected_edge_indices: Vec<usize>,
    edge_indices: Vec<usize>,
    direction: Vector3d,
    flip_direction: bool,
}

impl Face {
    fn new(
	vertex_indices: Vec<usize>,
	vertices: &[Vector3d],
	inside: &Vector3d,
	edges: &[Edge],
    ) -> Result<Self, String> {
	let face_vertex = &vertices[vertex_indices[0]];
	let (connected_edge_indices, edge_indices) = Self::get_edges(
	    &vertex_indices, edges,
	);
	let mut ret = Self {
	    direction: Vector3d::default(),
	    flip_direction: false,
	    vertex_indices,
	    connected_edge_indices,
	    edge_indices,
 	};
	ret.update(edges);
	if geometry::pos_raw_plane_signed_dist(
	    inside, face_vertex, ret.direction(),
	) > 0. {
	    ret.flip_direction = true;
	}
	if ret.direction.is_nan() {
	    Err("Face with undefined direction.".into())
	} else {
	    Ok(ret)
	}
    }

    pub fn direction(&self) -> &Vector3d {
	&self.direction
    }

    pub fn edge_indices(&self) -> &[usize] {
	&self.edge_indices
    }    

    pub fn connected_edge_indices(&self) -> &[usize] {
	&self.connected_edge_indices
    }

    pub fn enclosing_planes(
	&self, vertices: &[Vector3d], edges: &[Edge],
    ) -> Vec<EnclosingPlane> {
	let mut inside = Vector3d::default();
	for vertex_index in &self.vertex_indices[0..3] {
	    inside.add_assign(&vertices[*vertex_index]);
	}
	inside.scale_assign(1./3.);
	let mut enclosing_planes = Vec::with_capacity(edges.len());
	for edge_index in &self.edge_indices {
	    let edge = &edges[*edge_index];
	    let vertex_index = edge.start_index();
	    enclosing_planes.push(EnclosingPlane {
		vertex_index,
		direction: {
		    let mut ret = edge.direction.cross(&self.direction);
		    if geometry::pos_raw_plane_signed_dist(
			&inside, &vertices[vertex_index], &ret,
		    ) > 0. {ret.scale_assign(-1.);} 
		    ret
		}
	    })
	}
	enclosing_planes
    }
    
    pub fn vertex_indices(&self) -> &[usize] {
	&self.vertex_indices
    }

    fn get_edges(
	vertex_indices: &[usize],
	edges: &[Edge],
    ) -> (Vec<usize>, Vec<usize>) {
	let mut connected_edge_indices = Vec::new();
	let mut edge_indices = Vec::new();
	for (i, edge) in edges.iter().enumerate() {
	    let mut count = 0u32;
	    for vertex_index in vertex_indices {
		if *vertex_index == edge.start_index ||
		    *vertex_index == edge.end_index
		{
		    count += 1;
		    if count > 1 {break;}
		}
	    }
	    match count {
		1 => connected_edge_indices.push(i),
		2 => {
		    connected_edge_indices.push(i);
		    edge_indices.push(i);
		}
		_ => (),
	    }
	}
	(connected_edge_indices, edge_indices)
    }
    
    fn update(&mut self, edges: &[Edge]) {
	self.direction = edges[self.edge_indices[0]].direction
	    .cross(&edges[self.edge_indices[1]].direction)
	    .normal();
	if self.flip_direction {self.direction.scale_assign(-1.);}
    }
}

#[derive(Clone, Copy)]
pub struct EnclosingPlane {
    vertex_index: usize,
    direction: Vector3d,
}

impl EnclosingPlane {
    pub fn vertex_index(&self) -> usize {
	self.vertex_index
    }

    pub fn direction(&self) -> &Vector3d {
	&self.direction
    }
}
