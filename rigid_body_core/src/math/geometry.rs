#![allow(dead_code)]
use super::vector::{Vector2d, Vector3d, Vector4d};
use std::f64::EPSILON;

macro_rules! gen_finiteline {
    ($line:ident, $vector:ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $line {
            pub start: $vector,
            pub end: $vector,
        }

        impl $line {
            pub fn new(start: &$vector, end: &$vector) -> Self {
                Self {
                    start: *start,
                    end: *end,
                }
            }
        }
    };
}

gen_finiteline!(FiniteLine2d, Vector2d);
gen_finiteline!(FiniteLine3d, Vector3d);
gen_finiteline!(FiniteLine4d, Vector4d);

macro_rules! gen_infiniteline {
    ($line:ident, $vector:ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $line {
            pub pos: $vector,
            pub dir: $vector,
        }

        impl $line {
            pub fn new(pos: &$vector, dir: &$vector) -> Self {
                Self {
                    pos: *pos,
                    dir: *dir,
                }
            }
        }
    };
}

gen_infiniteline!(InfiniteLine3d, Vector3d);

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub pos: Vector3d,
    pub dir: Vector3d,
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub pos: Vector2d,
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub pos: Vector3d,
    pub radius: f64,
}

pub fn plane_line_intersection(
    plane: &Plane,
    line: &InfiniteLine3d,
) -> Option<Vector3d> {
    let dp = line.dir.dot(&plane.dir);
    if dp.abs() < EPSILON {
        return None;
    }
    Some(
        line.pos.add(
            &line
                .dir
                .scale(plane.pos.sub(&line.pos).dot(&plane.dir) / dp),
        ),
    )
}

pub fn plane_finite_line_intersection(
    plane: &Plane,
    pos_1: &Vector3d,
    pos_2: &Vector3d,
) -> Option<Vector3d> {
    raw_plane_finite_line_intersection(&plane.pos, &plane.dir, pos_1, pos_2)
}

pub fn pos_plane_signed_dist(pos: &Vector3d, plane: &Plane) -> f64 {
    pos_raw_plane_signed_dist(pos, &plane.pos, &plane.dir)
}

pub fn pos_raw_plane_dist(
    pos: &Vector3d,
    plane_pos: &Vector3d,
    plane_dir: &Vector3d,
) -> f64 {
    pos_raw_plane_signed_dist(pos, plane_pos, plane_dir).abs()
}

pub fn pos_raw_plane_signed_dist(
    pos: &Vector3d,
    plane_pos: &Vector3d,
    plane_dir: &Vector3d,
) -> f64 {
    plane_dir.dot(&pos.sub(plane_pos))
}

pub fn pos_plane_dist(pos: &Vector3d, plane: &Plane) -> f64 {
    pos_plane_signed_dist(pos, plane).abs()
}

pub fn pos_in_front_of_plane(pos: &Vector3d, plane: &Plane) -> bool {
    pos_plane_signed_dist(pos, plane) > 0.
}

pub fn raw_finite_line_closest_dist_sq(
    start_1: &Vector3d,
    end_1: &Vector3d,
    start_2: &Vector3d,
    end_2: &Vector3d,
) -> (Vector3d, Vector3d, f64) {
    fn clamp(f: f64, min: f64, max: f64) -> f64 {
        if f < min {
            min
        } else if f > max {
            max
        } else {
            f
        }
    }

    let dir_1 = end_1.sub(start_1);
    let dir_2 = end_2.sub(start_2);

    let mag_sq_1 = dir_1.mag_sq();
    let mag_sq_2 = dir_2.mag_sq();

    let start_diff = start_1.sub(start_2);
    let dp_2_sd = dir_2.dot(&start_diff);

    let mut l1;
    let mut l2;

    if mag_sq_1 <= 0. && mag_sq_2 <= 0. {
        return (*start_1, *start_2, start_1.dist_sq(start_2));
    }
    if mag_sq_1 <= 0. {
        l1 = 0.;
        l2 = clamp(dp_2_sd / mag_sq_2, 0., 1.);
    } else {
        let dp_1_sd = dir_1.dot(&start_diff);
        if mag_sq_2 <= 0. {
            l1 = clamp(-dp_1_sd / mag_sq_1, 0., 1.);
            l2 = 0.;
        } else {
            let dp_12 = dir_1.dot(&dir_2);
            let den = mag_sq_1 * mag_sq_2 - dp_12 * dp_12;
            if den < EPSILON {
                l1 = 0.
            } else {
                l1 =
                    clamp((dp_12 * dp_2_sd - dp_1_sd * mag_sq_2) / den, 0., 1.);
            }
            l2 = (dp_12 * l1 + dp_2_sd) / mag_sq_2;
            if l2 < 0. {
                l1 = clamp(-dp_1_sd / mag_sq_1, 0., 1.);
                l2 = 0.;
            } else if l2 > 1. {
                l1 = clamp((dp_12 - dp_1_sd) / mag_sq_1, 0., 1.);
                l2 = 1.;
            }
        }
    }
    let v1 = start_1.add(&dir_1.scale(l1));
    let v2 = start_2.add(&dir_2.scale(l2));
    (v1, v2, v1.sub(&v2).mag_sq())
}

pub fn raw_plane_finite_line_intersection(
    plane_pos: &Vector3d,
    plane_dir: &Vector3d,
    pos_1: &Vector3d,
    pos_2: &Vector3d,
) -> Option<Vector3d> {
    let line_dir = pos_2.sub(&pos_1);
    let dp = line_dir.dot(&plane_dir);
    if dp.abs() < EPSILON {
        return None;
    }
    let d = plane_pos.sub(&pos_1).dot(&plane_dir) / dp;
    if d < 0. || d > 1. {
        None
    } else {
        Some(pos_1.add(&line_dir.scale(d)))
    }
}
