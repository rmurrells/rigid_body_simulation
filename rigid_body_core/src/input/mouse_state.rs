#[derive(Clone, Copy, Default)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
    pub wheel_x: i32,
    pub wheel_y: i32,
    states: [bool; MouseButton::Last as usize],
}

impl MouseState {
    pub fn reset(&mut self) {
	self.xrel = 0;
	self.yrel = 0;
	self.wheel_y = 0;
    }

    pub fn set(&mut self, button: MouseButton, pressed: bool) {
        self.states[button as usize] = pressed;
    }

    pub fn get(&self, button: MouseButton) -> bool {
        self.states[button as usize]
    }
}

#[derive(Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Last,
}
