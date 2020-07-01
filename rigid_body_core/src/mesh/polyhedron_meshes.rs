#![allow(dead_code)]
use super::{
    Mesh,
    MeshTriangle,
};
use crate::math::vector::Vector3d;

pub fn cuboid(dim: &Vector3d) -> Mesh {
    let min = dim.scale(-0.5);
    let max = dim.scale(0.5);
    let vertices = vec![
	min,
	Vector3d::new(min[0], min[1], max[2]),
	Vector3d::new(min[0], max[1], min[2]),
	Vector3d::new(min[0], max[1], max[2]),
	max,
	Vector3d::new(max[0], max[1], min[2]),
	Vector3d::new(max[0], min[1], max[2]),
	Vector3d::new(max[0], min[1], min[2]),
    ];
    let mesh_triangles = vec![
	MeshTriangle::norm_from_vertices(&vertices, &[0, 2, 5]),
	MeshTriangle::norm_from_vertices(&vertices, &[0, 5, 7]),
	
	MeshTriangle::norm_from_vertices(&vertices, &[7, 5, 4]),
	MeshTriangle::norm_from_vertices(&vertices, &[7, 4, 6]),

	MeshTriangle::norm_from_vertices(&vertices, &[6, 4, 3]),
	MeshTriangle::norm_from_vertices(&vertices, &[6, 3, 1]),

	MeshTriangle::norm_from_vertices(&vertices, &[1, 3, 2]),
	MeshTriangle::norm_from_vertices(&vertices, &[1, 2, 0]),

	MeshTriangle::norm_from_vertices(&vertices, &[2, 3, 4]),
	MeshTriangle::norm_from_vertices(&vertices, &[2, 4, 5]),

	MeshTriangle::norm_from_vertices(&vertices, &[6, 1, 0]),
	MeshTriangle::norm_from_vertices(&vertices, &[6, 0, 7]),
    ];
    Mesh::new(vertices, mesh_triangles)
}

pub fn icosphere(radius: f64, mut n: u8) -> Mesh {
    let mut ret = regular_icosahedron(radius);
    while n > 0 {
	let mut temp = Vec::new();
	for mesh_triangle in &ret.mesh_triangles {
	    let vertices = &ret.vertices;
	    let v1 = &vertices[mesh_triangle.vertex_indices[0]];
	    let v2 = &vertices[mesh_triangle.vertex_indices[1]];
	    let v3 = &vertices[mesh_triangle.vertex_indices[2]];

	    let scale = |vector: &Vector3d| {
		let mut ret = vector.scale(0.5);
		ret.scale_assign(radius/ret.mag());
		ret
	    };
	    let v21 = scale(&v1.add(&v2));
	    let v32 = scale(&v2.add(&v3));
	    let v31 = scale(&v1.add(&v3));
	    ret.vertices.push(v21);
	    ret.vertices.push(v32);
	    ret.vertices.push(v31);
	    let vertices_len = ret.vertices.len();
	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices,
		&[
		    mesh_triangle.vertex_indices[0],
		    vertices_len-3,
		    vertices_len-1,
		],
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices,
		&[
		    vertices_len-3,
		    mesh_triangle.vertex_indices[1],
		    vertices_len-2,
		],
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices,
		&[
		    vertices_len-1,
		    vertices_len-2,
		    mesh_triangle.vertex_indices[2],
		],
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices,
		&[
		    vertices_len-3,
		    vertices_len-2,
		    vertices_len-1,
		],		
	    ));
	}
	ret.mesh_triangles = temp;
	n -= 1;
    }
    ret
}

pub fn regular_icosahedron(size: f64) -> Mesh {
    let golden = (1.+5f64.sqrt())*0.5;
    let w = (size*size/(1.+1./(golden*golden))).sqrt();
    let h = w/golden;
    
    let vertices = vec![
	Vector3d::new(-h, 0., w),
	Vector3d::new(h, 0., w),
	Vector3d::new(-h, 0., -w),
	Vector3d::new(h, 0., -w),

	Vector3d::new(0., w, h),
	Vector3d::new(0., w, -h),
	Vector3d::new(0., -w, h),
	Vector3d::new(0., -w, -h),
	
	Vector3d::new(w, h, 0.),
	Vector3d::new(-w, h, 0.),
	Vector3d::new(w, -h, 0.),
	Vector3d::new(-w, -h, 0.),
    ];

    let mesh_triangles = vec![
	MeshTriangle::norm_from_vertices(&vertices, &[4, 0, 1]),
	MeshTriangle::norm_from_vertices(&vertices, &[9, 0, 4]),
	MeshTriangle::norm_from_vertices(&vertices, &[5, 9, 4]),
	MeshTriangle::norm_from_vertices(&vertices, &[5, 4, 8]),
	MeshTriangle::norm_from_vertices(&vertices, &[8, 4, 1]),
	MeshTriangle::norm_from_vertices(&vertices, &[10, 8, 1]),
	MeshTriangle::norm_from_vertices(&vertices, &[3, 8, 10]),
	MeshTriangle::norm_from_vertices(&vertices, &[3, 5, 8]),
	MeshTriangle::norm_from_vertices(&vertices, &[2, 5, 3]),
	MeshTriangle::norm_from_vertices(&vertices, &[7, 2, 3]),
	MeshTriangle::norm_from_vertices(&vertices, &[10, 7, 3]),
	MeshTriangle::norm_from_vertices(&vertices, &[6, 7, 10]),
	MeshTriangle::norm_from_vertices(&vertices, &[11, 7, 6]),
	MeshTriangle::norm_from_vertices(&vertices, &[0, 11, 6]),
	MeshTriangle::norm_from_vertices(&vertices, &[1, 0, 6]),
	MeshTriangle::norm_from_vertices(&vertices, &[1, 6, 10]),
	MeshTriangle::norm_from_vertices(&vertices, &[0, 9, 11]),
	MeshTriangle::norm_from_vertices(&vertices, &[11, 9, 2]),
	MeshTriangle::norm_from_vertices(&vertices, &[2, 9, 5]),
	MeshTriangle::norm_from_vertices(&vertices, &[2, 7, 11]),
    ];
    Mesh::new(vertices, mesh_triangles)
}

pub fn regular_tetrahedron(size: f64) -> Mesh {
    let ot = 1./3.;
    let stt = (2f64/3.).sqrt();
    let stn = (2f64/9.).sqrt();
    let vertices = vec![
	Vector3d::new((8f64/9.).sqrt(), 0., -ot).scale(size),
	Vector3d::new(-stn, stt, -ot).scale(size),
	Vector3d::new(-stn, -stt, -ot).scale(size),
	Vector3d::new(0., 0., size),
    ];
    let mesh_triangles = vec![
	MeshTriangle::norm_from_vertices(&vertices, &[0, 2, 1]),
	MeshTriangle::norm_from_vertices(&vertices, &[0, 1, 3]),
	MeshTriangle::norm_from_vertices(&vertices, &[0, 3, 2]),
	MeshTriangle::norm_from_vertices(&vertices, &[1, 2, 3]),
    ];
    Mesh::new(vertices, mesh_triangles)
}
