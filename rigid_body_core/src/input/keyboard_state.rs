#[derive(Clone, Copy, Default)]
pub struct KeyboardState {
    states: [bool; Keycode::Last as usize],
}

impl KeyboardState {
    pub fn set(&mut self, keycode: Keycode, pressed: bool) {
        self.states[keycode as usize] = pressed;
    }

    pub fn get(&self, keycode: Keycode) -> bool {
        self.states[keycode as usize]
    }
}

#[derive(Copy, Clone)]
pub enum Keycode {
    A,
    D,
    E,
    Q,
    R,
    S,
    W,
    Z,
    Return,
    Space,
    Tab,
    Escape,
    LCtrl,
    LShift,
    Last,
}
