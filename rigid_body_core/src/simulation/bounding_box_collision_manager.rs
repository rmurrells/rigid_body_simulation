use super::{
    collision_table::CollisionTable,
    rigid_body::{BoundingBox, RigidBody},
};
use crate::utility::int_hash::IntSet;
use std::cmp::Ordering;

pub struct BoundingBoxCollisionManager {
    axes: [BoundingBoxAxisIntervals; 3],
}

impl BoundingBoxCollisionManager {
    pub fn new(collision_epsilon: f64) -> Self {
        Self {
            axes: [
                BoundingBoxAxisIntervals::new(0, collision_epsilon),
                BoundingBoxAxisIntervals::new(1, collision_epsilon),
                BoundingBoxAxisIntervals::new(2, collision_epsilon),
            ],
        }
    }

    pub fn generate(
        &mut self,
        rigid_bodies: &[RigidBody],
        collision_table: &mut CollisionTable,
    ) {
        for axis in &mut self.axes {
            axis.generate(rigid_bodies, collision_table);
        }
    }

    pub fn update(
        &mut self,
        rigid_bodies: &[RigidBody],
        collision_table: &mut CollisionTable,
    ) {
        for axis in &mut self.axes {
            axis.update(rigid_bodies, collision_table);
        }
    }
}

struct BoundingBoxAxisIntervals {
    axis: usize,
    sorted: Vec<(usize, usize)>,
    collision_epsilon: f64,
}

impl BoundingBoxAxisIntervals {
    fn new(axis: usize, collision_epsilon: f64) -> Self {
        Self {
            axis,
            sorted: Vec::new(),
            collision_epsilon,
        }
    }

    fn generate(
        &mut self,
        rigid_bodies: &[RigidBody],
        collision_table: &mut CollisionTable,
    ) {
        self.sorted.clear();
        for i in 0..rigid_bodies.len() {
            self.sorted.push((i, 0));
            self.sorted.push((i, 1));
        }
        let axis = self.axis;
        let collision_epsilon = self.collision_epsilon;
        self.sorted.sort_unstable_by(|a, b| {
            Self::get_order(axis, collision_epsilon, rigid_bodies, a, b)
        });

        let mut active = IntSet::default();
        for value in &self.sorted {
            if value.1 == 1 {
                active.remove(&value.0);
            } else {
                for j in &active {
                    collision_table.get_mut(value.0, *j).bounding_box
                        [self.axis] = true;
                }
                active.insert(value.0);
            }
        }
    }

    fn get_order(
        axis: usize,
        collision_epsilon: f64,
        rigid_bodies: &[RigidBody],
        a: &(usize, usize),
        b: &(usize, usize),
    ) -> Ordering {
        let get_bound = |bound_info: &(usize, usize)| {
            rigid_bodies[bound_info.0].bounding_box()[bound_info.1][axis]
                + match bound_info.1 {
                    0 => -collision_epsilon,
                    1 => collision_epsilon,
                    _ => unreachable!(),
                }
        };
        let mut order = get_bound(a).partial_cmp(&get_bound(b)).unwrap();
        if let Ordering::Equal = order {
            if a.1 == 0 && b.1 == 1 {
                order = Ordering::Greater;
            } else if a.1 == 1 && b.1 == 0 {
                order = Ordering::Less;
            }
        }
        order
    }

    fn overlap(
        &self,
        bounding_box_1: &BoundingBox,
        bounding_box_2: &BoundingBox,
    ) -> bool {
        !(bounding_box_1[1][self.axis] + self.collision_epsilon
            < bounding_box_2[0][self.axis] - self.collision_epsilon
            || bounding_box_1[0][self.axis] - self.collision_epsilon
                > bounding_box_2[1][self.axis] + self.collision_epsilon)
    }

    fn update(
        &mut self,
        rigid_bodies: &[RigidBody],
        collision_table: &mut CollisionTable,
    ) {
        for i in 1..self.sorted.len() {
            for j in (1..i + 1).rev() {
                match Self::get_order(
                    self.axis,
                    self.collision_epsilon,
                    rigid_bodies,
                    &self.sorted[j - 1],
                    &self.sorted[j],
                ) {
                    Ordering::Greater => {
                        self.sorted.swap(j - 1, j);
                        let index_1 = self.sorted[j - 1].0;
                        let index_2 = self.sorted[j].0;
                        collision_table
                            .get_mut(index_1, index_2)
                            .bounding_box[self.axis] = self.overlap(
                            rigid_bodies[index_1].bounding_box(),
                            rigid_bodies[index_2].bounding_box(),
                        );
                    }
                    _ => break,
                }
            }
        }
    }
}
