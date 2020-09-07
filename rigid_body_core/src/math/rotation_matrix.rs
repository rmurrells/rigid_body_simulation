#![allow(dead_code)]

use super::{
    matrix::{Matrix2x2, Matrix3x3},
    vector::Vector3d,
};

#[must_use]
pub fn two_dimensional(theta: f64) -> Matrix2x2 {
    let mut rot_mat = Matrix2x2::default();
    let cos = theta.cos();
    let sin = theta.sin();
    rot_mat[0][0] = cos;
    rot_mat[0][1] = -sin;
    rot_mat[1][0] = sin;
    rot_mat[1][1] = cos;
    rot_mat
}

#[must_use]
pub fn x(theta: f64) -> Matrix3x3 {
    let mut rot_mat = Matrix3x3::default();
    let cos = theta.cos();
    let sin = theta.sin();
    rot_mat[0][0] = 1.;
    rot_mat[1][1] = cos;
    rot_mat[1][2] = -sin;
    rot_mat[2][1] = sin;
    rot_mat[2][2] = cos;
    rot_mat
}

#[must_use]
pub fn y(theta: f64) -> Matrix3x3 {
    let mut rot_mat = Matrix3x3::default();
    let cos = theta.cos();
    let sin = theta.sin();
    rot_mat[0][0] = cos;
    rot_mat[0][2] = sin;
    rot_mat[1][1] = 1.;
    rot_mat[2][0] = -sin;
    rot_mat[2][2] = cos;
    rot_mat
}

#[must_use]
pub fn z(theta: f64) -> Matrix3x3 {
    let mut rot_mat = Matrix3x3::default();
    let cos = theta.cos();
    let sin = theta.sin();
    rot_mat[0][0] = cos;
    rot_mat[0][1] = -sin;
    rot_mat[1][0] = sin;
    rot_mat[1][1] = cos;
    rot_mat[2][2] = 1.;
    rot_mat
}

#[must_use]
pub fn general(dir: &Vector3d, theta: f64) -> Matrix3x3 {
    let mut rot_mat = Matrix3x3::default();
    let cos = theta.cos();
    let m_cos = 1. - cos;
    let xy = dir[0] * dir[1] * m_cos;
    let xz = dir[0] * dir[2] * m_cos;
    let yz = dir[1] * dir[2] * m_cos;
    let sin = theta.sin();
    let x_sin = dir[0] * sin;
    let y_sin = dir[1] * sin;
    let z_sin = dir[2] * sin;
    rot_mat[0][0] = cos + dir[0] * dir[0] * m_cos;
    rot_mat[0][1] = xy - z_sin;
    rot_mat[0][2] = xz + y_sin;
    rot_mat[1][0] = xy + z_sin;
    rot_mat[1][1] = cos + dir[1] * dir[1] * m_cos;
    rot_mat[1][2] = yz - x_sin;
    rot_mat[2][0] = xz - y_sin;
    rot_mat[2][1] = yz + x_sin;
    rot_mat[2][2] = cos + dir[2] * dir[2] * m_cos;
    rot_mat
}
