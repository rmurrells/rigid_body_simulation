#![allow(dead_code)]
use crate::{
    math::vector::Vector3d,
    render::Camera,
};
use super::{
    InputCore,
    keyboard_state::Keycode,
    mouse_state::{
	MouseButton,
	MouseState,
    },
};

pub struct CameraMover {
    pub center: Vector3d,
    pub camera_range: (f64, f64),
    pub theta_scale: f64,
    pub wheel_scale: f64,
    pub move_fact: f64,
    pub mode: CameraMode,
}

impl CameraMover {
    pub fn move_camera(&self, input: &InputCore, camera: &mut Camera) {
	match self.mode {
	    CameraMode::Rel => self.move_rel(&input.mouse_state, camera),
	    CameraMode::Fps => self.move_fps(input, camera),
	}
    }
    
    fn move_rel(&self, mouse_state: &MouseState, camera: &mut Camera) {
	let mut dist = self.center.dist(&camera.position)
	    -f64::from(mouse_state.wheel_y)*self.wheel_scale;
	if dist < self.camera_range.0 {
	    dist = self.camera_range.0;
	} else if dist > self.camera_range.1 {
	    dist = self.camera_range.1;
	}
	
	if mouse_state.get(MouseButton::Left) {
	    camera.rotate(
		-f64::from(mouse_state.yrel)*self.theta_scale,
		f64::from(mouse_state.xrel)*self.theta_scale,
	    );
        }
	camera.position = self.center.sub(
	    &camera.dir().scale(dist),
	);
    }

    fn move_fps(&self, input: &InputCore, camera: &mut Camera) {
	if input.mouse_state.get(MouseButton::Left) {
	    camera.rotate(
		-f64::from(input.mouse_state.yrel)*self.theta_scale,
		f64::from(input.mouse_state.xrel)*self.theta_scale,
	    );
	}
	
	let move_lin =
	    (input.keyboard_state.get(Keycode::W) as i32
	     -input.keyboard_state.get(Keycode::S) as i32) as f64
	    *self.move_fact;
	let move_hor =
	    (input.keyboard_state.get(Keycode::D) as i32
	     -input.keyboard_state.get(Keycode::A) as i32) as f64
	    *self.move_fact;
	let move_vert =
	    (input.keyboard_state.get(Keycode::Q) as i32
	     -input.keyboard_state.get(Keycode::E) as i32) as f64
	    *self.move_fact;
	
	let theta_y = camera.theta_y();
	let sin = theta_y.sin();
	let cos = theta_y.cos();
	camera.position = camera.position.add(&Vector3d::new(
	    -sin*move_lin-cos*move_hor,
            move_vert,
	    cos*move_lin-sin*move_hor,
	));
    }
}

pub enum CameraMode {
    Rel,
    Fps,
}
