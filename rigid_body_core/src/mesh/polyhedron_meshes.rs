#![allow(dead_code)]
use super::{
    Mesh,
    MeshTriangle,
};
use crate::math::vector::Vector3d;

pub fn cuboid(dim: &Vector3d) -> Mesh {
    let min = dim.scale(-0.5);
    let max = dim.scale(0.5);
    Mesh::from(vec![	
	MeshTriangle::norm_from_vertices(
	    &min,
	    &Vector3d::new(min[0], max[1], min[2]),
	    &Vector3d::new(max[0], max[1], min[2]),
	),
	MeshTriangle::norm_from_vertices(
	    &min,
	    &Vector3d::new(max[0], max[1], min[2]),
	    &Vector3d::new(max[0], min[1], min[2]),
	),
	
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(max[0], min[1], min[2]),
	    &Vector3d::new(max[0], max[1], min[2]),
	    &max,
	),
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(max[0], min[1], min[2]),
	    &max,
	    &Vector3d::new(max[0], min[1], max[2]),
	),
	
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(max[0], min[1], max[2]),
	    &max,
	    &Vector3d::new(min[0], max[1], max[2]),
	),
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(max[0], min[1], max[2]),
	    &Vector3d::new(min[0], max[1], max[2]),
	    &Vector3d::new(min[0], min[1], max[2]),
	),
	
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(min[0], min[1], max[2]),
	    &Vector3d::new(min[0], max[1], max[2]),
	    &Vector3d::new(min[0], max[1], min[2]),
	),
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(min[0], min[1], max[2]),
	    &Vector3d::new(min[0], max[1], min[2]),
	    &min,
	),
	
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(min[0], max[1], min[2]),
	    &Vector3d::new(min[0], max[1], max[2]),
	    &max,
	),
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(min[0], max[1], min[2]),
	    &max,
	    &Vector3d::new(max[0], max[1], min[2]),
	),
	
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(max[0], min[1], max[2]),
	    &Vector3d::new(min[0], min[1], max[2]),
	    &min,
	),
	MeshTriangle::norm_from_vertices(
	    &Vector3d::new(max[0], min[1], max[2]),
	    &min,
	    &Vector3d::new(max[0], min[1], min[2]),
	),
    ])
}

pub fn icosphere(radius: f64, mut n: u8) -> Mesh {
    let mut ret = regular_icosahedron(radius);
    while n > 0 {
	let mut temp = Vec::new();
	for mesh_triangle in &ret.mesh_triangles {
	    let vertices = &mesh_triangle.triangle_3d.vertices;
	    let v1 = &vertices[0];
	    let v2 = &vertices[1];
	    let v3 = &vertices[2];

	    let scale = |vector: &Vector3d| {
		let mut ret = vector.scale(0.5);
		ret.scale_assign(radius/ret.mag());
		ret
	    };
	    let v21 = scale(&v1.add(&v2));
	    let v32 = scale(&v2.add(&v3));
	    let v31 = scale(&v1.add(&v3));
	    temp.push(MeshTriangle::norm_from_vertices(
		v1, &v21, &v31,
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&v21, v2, &v32,
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&v31, &v32, v3,
	    ));
	    temp.push(MeshTriangle::norm_from_vertices(
		&v21, &v32, &v31,
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
    
    let vertices = [
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

    Mesh::from(vec![
	MeshTriangle::norm_from_vertices(
	    &vertices[4], &vertices[0], &vertices[1],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[9], &vertices[0], &vertices[4],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[5], &vertices[9], &vertices[4],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[5], &vertices[4], &vertices[8],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[8], &vertices[4], &vertices[1],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[10], &vertices[8], &vertices[1],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[3], &vertices[8], &vertices[10],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[3], &vertices[5], &vertices[8],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[2], &vertices[5], &vertices[3],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[7], &vertices[2], &vertices[3],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[10], &vertices[7], &vertices[3],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[6], &vertices[7], &vertices[10],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[11], &vertices[7], &vertices[6],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[0], &vertices[11], &vertices[6],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[1], &vertices[0], &vertices[6],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[1], &vertices[6], &vertices[10],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[0], &vertices[9], &vertices[11],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[11], &vertices[9], &vertices[2],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[2], &vertices[9], &vertices[5],
	),
	MeshTriangle::norm_from_vertices(
	    &vertices[2], &vertices[7], &vertices[11],
	),
    ])
}
