#![allow(dead_code)]
use super::{
    matrix::{Matrix2x2, Matrix3x3, Matrix4x4},
    vector::{Vector2d, Vector3d, Vector4d},
};

macro_rules! get_mat_value {
    ($mat:ident, $i:ident, $j:ident, nottranspose) => {
        $mat[$i][$j]
    };

    ($mat:ident, $i:ident, $j:ident, transpose) => {
        $mat[$j][$i]
    };
}

macro_rules! gen_mult_fn {
    ($name:ident, $matrix:ident, $vector:ident) => {
        gen_mult_fn!($name, $matrix, $vector, nottranspose);
    };

    ($name:ident, $matrix:ident, $vector:ident, $opt:ident) => {
        #[must_use]
        pub fn $name(mat: &$matrix, vec: &$vector) -> $vector {
            let mut ret = $vector::default();
            let len = vec.len();
            for i in 0..len {
                for j in 0..len {
                    ret[i] += get_mat_value!(mat, i, j, $opt) * vec[j];
                }
            }
            ret
        }
    };
}

gen_mult_fn!(mult_2, Matrix2x2, Vector2d);
gen_mult_fn!(mult_2t, Matrix2x2, Vector2d, transpose);
gen_mult_fn!(mult_3, Matrix3x3, Vector3d);
gen_mult_fn!(mult_3t, Matrix3x3, Vector3d, transpose);
gen_mult_fn!(mult_4, Matrix4x4, Vector4d);
