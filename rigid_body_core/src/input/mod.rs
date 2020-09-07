pub mod camera_mover;
pub mod keyboard_state;
pub mod mouse_state;

use keyboard_state::{KeyboardState, Keycode};
use mouse_state::{MouseButton, MouseState};

pub struct InputCore {
    pub keyboard_state: KeyboardState,
    pub mouse_state: MouseState,
    pub advance_simulation: bool,
    pub reset: bool,
    pub tick: bool,
    pub quit: bool,
}

impl InputCore {
    pub fn new() -> Self {
        Self {
            keyboard_state: KeyboardState::default(),
            mouse_state: MouseState::default(),
            advance_simulation: true,
            reset: false,
            tick: false,
            quit: false,
        }
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::KeyDown { key } => self.keyboard_state.set(key, true),
            InputEvent::KeyUp { key } => self.key_up(key),
            InputEvent::MouseButtonDown { button } => {
                self.mouse_state.set(button, true)
            }
            InputEvent::MouseButtonUp { button } => {
                self.mouse_state.set(button, false)
            }
            InputEvent::MouseMotion { x, y, xrel, yrel } => {
                self.mouse_state.x = x;
                self.mouse_state.y = y;
                self.mouse_state.xrel = xrel;
                self.mouse_state.yrel = yrel;
            }
            InputEvent::MouseWheel { xrel, yrel } => {
                self.mouse_state.wheel_x = xrel;
                self.mouse_state.wheel_y = yrel;
            }
            InputEvent::Quit => self.quit = true,
        }
    }

    pub fn clear(&mut self) {
        self.mouse_state.reset();
        self.reset = false;
        self.tick = false;
    }

    pub fn key_up(&mut self, key: Keycode) {
        if self.keyboard_state.get(key) {
            match key {
                Keycode::Escape => self.quit = true,
                Keycode::R => self.reset = true,
                Keycode::Return => self.tick = true,
                Keycode::Space => {
                    self.advance_simulation = !self.advance_simulation
                }
                _ => (),
            }
        }
        self.keyboard_state.set(key, false);
    }
}

impl Default for InputCore {
    fn default() -> Self {
        Self::new()
    }
}

pub enum InputEvent {
    KeyDown {
        key: Keycode,
    },
    KeyUp {
        key: Keycode,
    },
    MouseButtonDown {
        button: MouseButton,
    },
    MouseButtonUp {
        button: MouseButton,
    },
    MouseMotion {
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    },
    MouseWheel {
        xrel: i32,
        yrel: i32,
    },
    Quit,
}
