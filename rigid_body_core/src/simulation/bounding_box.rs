use crate::{
    math::{
	matrix::Matrix3x3,
	vector::Vector3d,
    },
    UID,
};
use super::{
    rigid_body::{
	self,
	RigidBody,
    },
};

pub struct BoundingBox {
    pub uid: UID,
    pub inner_opt: Option<BoundingBoxInner>,
}

impl BoundingBox {
    pub fn new() -> Self {
	Self {
	    uid: crate::get_new_uid(),
	    inner_opt: None,
	}
    }

    pub fn set(
	&mut self,
	dimensions_opt: &Option<(&Vector3d, &Vector3d)>,
	initial_rigid_bodies: &mut Vec<RigidBody>,
	rigid_bodies: &mut Vec<RigidBody>,
    ) {
	if let Some(bounding_box) = &mut self.inner_opt {
	    let mut i = 0;
	    while i < rigid_bodies.len() &&
		!bounding_box.rigid_body_uids.is_empty()
	    {
		if let Some(index) = bounding_box.rigid_body_uids.iter()
		    .position(|e| *e == rigid_bodies[i].uid())
		{
		    bounding_box.rigid_body_uids.swap_remove(index);
		    initial_rigid_bodies.swap_remove(i);
		    rigid_bodies.swap_remove(i);
		} else {
		    i += 1;
		}
	    }
	    self.inner_opt = None;
	}
	if let Some((min, max)) = &dimensions_opt {
	    let position = max.add(min).scale(0.5);
	    let x = position[0];
	    let y = position[1];
	    let z = position[2];
	    let dimensions = max.sub(min);
	    let xd = dimensions[0];
	    let yd = dimensions[1];
	    let zd = dimensions[2];
	    let mut rigid_body_uids = Vec::with_capacity(6);
	    let mut add_bounding_box_cuboid = |position: &Vector3d| {
		let cuboid = RigidBody::cuboid(
		    &dimensions,
		    0.,
		    position,
		    &Matrix3x3::identity(),
		    &Vector3d::default(),
		    &Vector3d::default(),
		);
		rigid_body_uids.push(cuboid.uid());
		initial_rigid_bodies.push(cuboid.clone());
		rigid_bodies.push(cuboid);
	    };
	    add_bounding_box_cuboid(&Vector3d::new(x, y, z-zd));
	    add_bounding_box_cuboid(&Vector3d::new(x, y, z+zd));
	    add_bounding_box_cuboid(&Vector3d::new(x, y-yd, z));
	    add_bounding_box_cuboid(&Vector3d::new(x, y+yd, z));
	    add_bounding_box_cuboid(&Vector3d::new(x-xd, y, z));
	    add_bounding_box_cuboid(&Vector3d::new(x+xd, y, z));
	    self.inner_opt = Some(BoundingBoxInner {
		dimensions: [**min, **max], rigid_body_uids,
	    });
	}
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
	Self::new()
    }
}

pub struct BoundingBoxInner {
    pub dimensions: rigid_body::BoundingBox,
    pub rigid_body_uids: Vec<usize>,
}

impl BoundingBoxInner {
    pub fn contain(&self, rigid_bodies: &mut [RigidBody]) {
	for rigid_body in rigid_bodies {
	    if rigid_body.is_immovable() {continue;}
	    for axis in 0..3 {
		let min_dist = rigid_body.bounding_box()[0][axis]
		    -self.dimensions[0][axis];
		if min_dist < 0. {
		    rigid_body.position[axis] -= min_dist;
		    rigid_body.update_geometry();
		    continue;
		}
		let max_dist = rigid_body.bounding_box()[1][axis]
		    -self.dimensions[1][axis];
		if max_dist > 0. {
		    rigid_body.position[axis] -= max_dist;
		    rigid_body.update_geometry();
		}
	    }
	}
    }
}
