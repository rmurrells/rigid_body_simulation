#![allow(dead_code)]
use std::{
    default::Default,
    f64::EPSILON,
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

macro_rules! gen_vector {
    ($vector:ident, $n:expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $vector {
            v: [f64; $n],
        }

        impl $vector {
            pub fn add(&self, other: &Self) -> Self {
                let mut ret = *self;
                ret.add_assign(other);
                ret
            }

            pub fn add_assign(&mut self, other: &Self) {
                for (i, j) in self.iter_mut().zip(other.iter()) {
                    *i += j;
                }
            }

            pub fn dir(&self) -> Self {
                self.scale(1. / self.mag())
            }

            pub fn dist(&self, other: &Self) -> f64 {
                self.dist_sq(other).sqrt()
            }

            pub fn dist_sq(&self, other: &Self) -> f64 {
                let mut ret = 0.;
                for (i, j) in self.iter().zip(other.iter()) {
                    let dist = i - j;
                    ret += dist * dist;
                }
                ret
            }

            pub fn dot(&self, other: &Self) -> f64 {
                let mut ret = 0.;
                for (i, j) in self.iter().zip(other.iter()) {
                    ret += i * j;
                }
                ret
            }

            pub fn is_nan(&self) -> bool {
                for i in self.iter() {
                    if i.is_nan() {
                        return true;
                    }
                }
                false
            }

            pub fn is_zero(&self) -> bool {
                for i in self.iter() {
                    if i.abs() > EPSILON {
                        return false;
                    }
                }
                true
            }

            pub fn iter(&self) -> Iter<f64> {
                self.v.iter()
            }

            pub fn iter_mut(&mut self) -> IterMut<f64> {
                self.v.iter_mut()
            }

            pub fn len(&self) -> usize {
                self.v.len()
            }

            pub fn mag(&self) -> f64 {
                self.mag_sq().sqrt()
            }

            pub fn mag_sq(&self) -> f64 {
                let mut ret = 0.;
                for val in self.iter() {
                    ret += val * val;
                }
                ret
            }

            pub fn normal(&self) -> Self {
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
                for i in self.iter_mut() {
                    *i *= factor;
                }
            }

            #[must_use]
            pub fn sub(&self, other: &Self) -> Self {
                let mut ret = *self;
                ret.sub_assign(other);
                ret
            }

            pub fn sub_assign(&mut self, other: &Self) {
                for (i, j) in self.iter_mut().zip(other.iter()) {
                    *i -= j;
                }
            }
        }

        impl Default for $vector {
            fn default() -> Self {
                Self { v: [0.; $n] }
            }
        }

        impl Display for $vector {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{:?}", self.v)
            }
        }

        impl Index<usize> for $vector {
            type Output = f64;
            fn index(&self, index: usize) -> &Self::Output {
                &self.v[index]
            }
        }

        impl IndexMut<usize> for $vector {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.v[index]
            }
        }
    };
}

gen_vector!(Vector2d, 2);
gen_vector!(Vector3d, 3);
gen_vector!(Vector4d, 4);

macro_rules! gen_vector_new {
    ($vector:ident, [$($d:ident),*]) => {
	impl $vector {
	    pub fn new($($d: f64,)*) -> Self {
		Self{v: [$($d,)*]}
	    }
	}
    }
}

gen_vector_new!(Vector2d, [x, y]);
gen_vector_new!(Vector3d, [x, y, z]);
gen_vector_new!(Vector4d, [x, y, z, w]);

impl Vector3d {
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }
}
