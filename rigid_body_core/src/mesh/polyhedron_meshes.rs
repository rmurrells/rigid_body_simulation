#![allow(dead_code)]
use super::{
    Mesh,
    MeshTriangle,
};
use crate::math::vector::Vector3d;
use std::collections::HashMap;

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
    let mut vertex_map = HashMap::<(usize, usize), usize>::default();
    while n > 0 {
	let mut temp = Vec::new();
	for mesh_triangle in &ret.mesh_triangles {
	    let mut bisect = |va: usize, vb: usize, vertices: &mut Vec<Vector3d>| {
		if let Some(vi) = vertex_map.get(&(va, vb)) {
		    *vi
		} else if let Some(vi) = vertex_map.get(&(vb, va)) {
		    *vi
		} else {
		    let mut vertex = vertices[va].add(&vertices[vb]).scale(0.5);
		    vertex.scale_assign(radius/vertex.mag());
		    let len = vertices.len();
		    vertex_map.insert((va, vb), len);
		    vertices.push(vertex);
		    len
		}
	    };
	    let v1 = mesh_triangle.vertex_indices[0];
	    let v2 = mesh_triangle.vertex_indices[1];
	    let v3 = mesh_triangle.vertex_indices[2];
	    let v21 = bisect(v1, v2, &mut ret.vertices);
	    let v32 = bisect(v2, v3, &mut ret.vertices);
	    let v31 = bisect(v1, v3, &mut ret.vertices);

	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices, &[v1, v21, v31]
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices, &[v21, v2, v32],
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices, &[v31, v32, v3],
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&ret.vertices, &[v21, v32, v31],
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
