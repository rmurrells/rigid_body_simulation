#[derive(Default)]
pub struct KeyStates {
    states: [bool; Keycode::Last as usize],
}

impl KeyStates {
    pub fn set(&mut self, keycode: Keycode, pressed: bool) {
        self.states[keycode as usize] = pressed;
    }

    pub fn get(&self, keycode: Keycode) -> bool {
        self.states[keycode as usize]
    }
}

pub enum Keycode {
    A,
    D,
    E,
    Q,
    R,
    S,
    W,
    Return,
    Space,
    Escape,
    LCtrl,
    LShift,
    Last,
}
