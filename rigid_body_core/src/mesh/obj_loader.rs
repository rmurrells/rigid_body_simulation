#![allow(dead_code)]
use crate::math::vector::Vector3d;
use super::{
    Mesh,
    MeshTriangle,
};
use std::{
    fs::File,
    io::{
	BufRead,
	BufReader,
	Error,
	ErrorKind,
	Result,
    },
    path::Path,
};

pub fn simple<P: AsRef<Path>>(path: P) -> Result<Mesh> {
    fn get_error(msg: &str) -> Error {
	Error::new(ErrorKind::Other, msg)
    }
    if let Some(os_str) = path.as_ref().extension() {
	if os_str != "obj" {
	    return Err(get_error("Requires an .obj file."));
	}
    } else {
	return Err(get_error("Requires an .obj file."));
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut vertices = Vec::new();
    let mut mesh_triangles = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
	let mut line_iter = line.split_whitespace();
	if let Some(first) = line_iter.next() {
	    if first == "v" {
		let values = line_iter
		    .map(|s| s.parse::<f64>().expect("Could not parse file line to f64."))
		    .collect::<Vec<f64>>();
		if values.len() != 3 {
		    return Err(get_error("Unrecognized file format."));
		}
		vertices.push(Vector3d::new(
		    values[0], values[1], values[2],
		));	
	    } else if first == "f" {
		let values = line_iter
		    .map(|s| s.parse::<usize>().expect("Could not parse file line to usize.")-1)
		    .collect::<Vec<usize>>();
		if values.len() != 3 {
		    return Err(get_error("Unrecognized file format."));
		}
		mesh_triangles.push(MeshTriangle::norm_from_vertices(
		    &vertices, &[values[0], values[1], values[2]],
		));
	    }
	}
    }
    Ok(Mesh::new(vertices, mesh_triangles))
}
