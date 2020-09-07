#![allow(dead_code)]
use crate::math::{
    matrix::Matrix3x3, matrix_vector, rotation_matrix, vector::Vector3d,
};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vector3d,
    theta_x: f64,
    theta_x_max: f64,
    theta_y: f64,
    theta_y_max: f64,
    rotation_matrix: Matrix3x3,
    direction: Vector3d,
}

impl Camera {
    pub fn new(position: &Vector3d, theta_x: f64, theta_y: f64) -> Self {
        let mut camera = Self {
            position: *position,
            theta_x,
            theta_x_max: PI / 2.,
            theta_y,
            theta_y_max: PI * 2.,
            rotation_matrix: Matrix3x3::default(),
            direction: Vector3d::default(),
        };
        camera.update();
        camera
    }

    pub fn dir(&self) -> &Vector3d {
        &self.direction
    }

    pub fn theta_x(&self) -> f64 {
        self.theta_x
    }

    pub fn theta_y(&self) -> f64 {
        self.theta_y
    }

    pub fn rotate(&mut self, theta_x: f64, theta_y: f64) {
        self.theta_x += theta_x;
        if self.theta_x > self.theta_x_max {
            self.theta_x = self.theta_x_max;
        } else if self.theta_x < -self.theta_x_max {
            self.theta_x = -self.theta_x_max;
        }

        self.theta_y += theta_y;
        if self.theta_y > self.theta_y_max {
            self.theta_y -= self.theta_y_max;
        } else if self.theta_y < -self.theta_y_max {
            self.theta_y += self.theta_y_max;
        }

        self.update();
    }

    pub fn to_world(&self, position: &Vector3d) -> Vector3d {
        matrix_vector::mult_3t(&self.rotation_matrix, &position)
            .add(&self.position)
    }

    pub fn view(&self, position: &Vector3d) -> Vector3d {
        matrix_vector::mult_3(
            &self.rotation_matrix,
            &position.sub(&self.position),
        )
    }

    fn update(&mut self) {
        self.update_rotation_matrix();
        self.update_direction();
    }

    fn update_rotation_matrix(&mut self) {
        self.rotation_matrix = rotation_matrix::x(self.theta_x)
            .mult(&rotation_matrix::y(self.theta_y));
    }

    fn update_direction(&mut self) {
        let theta_x_rot = self.theta_x - PI / 2.;
        let theta_x_rot_sin = theta_x_rot.sin();
        self.direction = Vector3d::new(
            theta_x_rot_sin * self.theta_y.sin(),
            theta_x_rot.cos(),
            -theta_x_rot_sin * self.theta_y.cos(),
        );
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(&Vector3d::default(), 0., 0.)
    }
}
