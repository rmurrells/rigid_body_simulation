#[derive(Default)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
    pub wheel_y: i32,
    pub left: bool,
    pub middle: bool,
    pub right: bool
}

impl MouseState {
    pub fn reset(&mut self) {
	self.xrel = 0;
	self.yrel = 0;
	self.wheel_y = 0;
    }
}
