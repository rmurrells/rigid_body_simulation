use super::{matrix::Matrix3x3, vector::Vector3d};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy)]
pub struct Quarternion {
    s: f64,
    v: Vector3d,
}

impl Quarternion {
    pub fn new(s: f64, v: &Vector3d) -> Self {
        Self { s, v: *v }
    }

    pub fn from_matrix(matrix: &Matrix3x3) -> Self {
        let trace = matrix.trace();
        let mut ret = Self::default();
        if trace >= 0. {
            let mut s = (trace + 1.).sqrt();
            ret.s = 0.5 * s;
            s = 0.5 / s;
            ret.v[0] = (matrix[2][1] - matrix[1][2]) * s;
            ret.v[1] = (matrix[0][2] - matrix[2][0]) * s;
            ret.v[2] = (matrix[1][0] - matrix[0][1]) * s;
        } else {
            let mut i = 0;
            if matrix[1][1] > matrix[0][0] {
                i = 1;
            }
            if matrix[2][2] > matrix[i][i] {
                i = 2;
            }
            match i {
                0 => {
                    let mut s = (matrix[0][0] - (matrix[1][1] + matrix[2][2])
                        + 1.)
                        .sqrt();
                    ret.v[0] = 0.5 * s;
                    s = 0.5 / s;
                    ret.v[1] = (matrix[0][1] + matrix[1][0]) * s;
                    ret.v[2] = (matrix[2][0] + matrix[0][2]) * s;
                    ret.s = (matrix[2][1] - matrix[1][2]) * s;
                }
                1 => {
                    let mut s = (matrix[1][1] - (matrix[2][2] + matrix[0][0])
                        + 1.)
                        .sqrt();
                    ret.v[1] = 0.5 * s;
                    s = 0.5 / s;
                    ret.v[2] = (matrix[1][2] + matrix[2][1]) * s;
                    ret.v[0] = (matrix[0][1] + matrix[1][0]) * s;
                    ret.s = (matrix[0][2] - matrix[2][0]) * s;
                }
                2 => {
                    let mut s = (matrix[2][2] - (matrix[0][0] + matrix[1][1])
                        + 1.)
                        .sqrt();
                    ret.v[2] = 0.5 * s;
                    s = 0.5 / s;
                    ret.v[0] = (matrix[2][0] + matrix[0][2]) * s;
                    ret.v[1] = (matrix[1][2] + matrix[2][1]) * s;
                    ret.s = (matrix[1][0] - matrix[0][1]) * s;
                }
                _ => unreachable!(),
            }
        }
        ret
    }

    pub fn to_matrix(&self) -> Matrix3x3 {
        let s2 = 2. * self.s;
        let vx = self.v[0];
        let vy = self.v[1];
        let vz = self.v[2];
        let vx22 = 2. * vx * vx;
        let vy22 = 2. * vy * vy;
        let vz22 = 2. * vz * vz;
        let vx_vy2 = 2. * vx * vy;
        let vy_vz2 = 2. * vy * vz;
        let vx_vz2 = 2. * vx * vz;
        Matrix3x3::new(&[
            [1. - vy22 - vz22, vx_vy2 - s2 * vz, vx_vz2 + s2 * vy],
            [vx_vy2 + s2 * vz, 1. - vx22 - vz22, vy_vz2 - s2 * vx],
            [vx_vz2 - s2 * vy, vy_vz2 + s2 * vx, 1. - vx22 - vy22],
        ])
    }

    pub fn add_assign(&mut self, other: &Self) {
        self.s += other.s;
        self.v.add_assign(&other.v);
    }

    pub fn mag(&self) -> f64 {
        self.mag_sq().sqrt()
    }

    pub fn mag_sq(&self) -> f64 {
        self.s * self.s + self.v.mag_sq()
    }

    pub fn mult(&self, other: &Self) -> Self {
        Self::new(
            self.s * other.s - self.v.dot(&other.v),
            &other
                .v
                .scale(self.s)
                .add(&self.v.scale(other.s))
                .add(&self.v.cross(&other.v)),
        )
    }

    pub fn normal(&mut self) -> Self {
        let mut ret = *self;
        ret.normalize();
        ret
    }

    pub fn normalize(&mut self) {
        self.scale_assign(1. / self.mag());
    }

    pub fn scale(&self, factor: f64) -> Self {
        let mut ret = *self;
        ret.scale_assign(factor);
        ret
    }

    pub fn scale_assign(&mut self, factor: f64) {
        self.s *= factor;
        self.v.scale_assign(factor);
    }

    pub fn sub_assign(&mut self, other: &Self) {
        self.s -= other.s;
        self.v.sub_assign(&other.v);
    }
}

impl Default for Quarternion {
    fn default() -> Self {
        Self {
            s: 0.,
            v: Vector3d::default(),
        }
    }
}

impl Display for Quarternion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.s, self.v)
    }
}
