mod key_states;
mod mouse_state;

pub mod camera_mover;

use crate::StrResult;
use sdl2::{
    event::Event,
    EventPump,
    keyboard,
    mouse::MouseButton,
    Sdl,
};

pub use key_states::{
    Keycode,
    KeyStates,
};
pub use mouse_state::MouseState;

pub struct Input {
    event_pump: EventPump,
    pub key_states: KeyStates,
    pub mouse_state: MouseState,
}

impl Input {
    pub fn new(context: &Sdl) -> StrResult<Self> {
        Ok(Input{
	    event_pump: context.event_pump()?,
	    key_states: KeyStates::default(),
	    mouse_state: MouseState::default(),
	})
    }

    pub fn get(&mut self) -> InputState {
	self.mouse_state.reset();
        for event in self.event_pump.poll_iter() {
            match event {
		Event::KeyDown{keycode: Some(key), ..} => {
		    Self::set_key_states(key, true, &mut self.key_states);
		    match key {
			keyboard::Keycode::Return => return InputState::Tick,
			_ => (),
		    }
		}
		Event::KeyUp{keycode: Some(key), ..} => {
		    Self::set_key_states(key, false, &mut self.key_states);
		    match key {
			keyboard::Keycode::Escape => return InputState::Quit,
			keyboard::Keycode::R => return InputState::Reset,
			keyboard::Keycode::Space => return InputState::Pause,
			_ => (),
		    }
		}
		Event::MouseMotion{xrel, yrel, ..} => {
		    self.mouse_state.xrel = xrel;
		    self.mouse_state.yrel = yrel;
		},
		Event::MouseButtonDown{mouse_btn, ..} => {
                    match mouse_btn {
			MouseButton::Left => self.mouse_state.left = true,
			_ => (),
                    }
		},
		Event::MouseButtonUp{mouse_btn, ..} => {
                    match mouse_btn {
			MouseButton::Left => self.mouse_state.left = false,
			_ => (),
                    }
		},
		Event::MouseWheel{y, ..} => {
		    self.mouse_state.wheel_y = y;
                }
                Event::Quit{..} => {
                    return InputState::Quit;
                },
                _ => (),
            }
        }
	InputState::Continue
    }

    fn set_key_states(
	key: keyboard::Keycode,
	key_down: bool,
	key_states: &mut KeyStates,
    ) {
	match key {
	    keyboard::Keycode::A =>
		key_states.set(key_states::Keycode::A, key_down),
	    keyboard::Keycode::D =>
		key_states.set(key_states::Keycode::D, key_down),
	    keyboard::Keycode::E =>
		key_states.set(key_states::Keycode::E, key_down),
	    keyboard::Keycode::Q =>
		key_states.set(key_states::Keycode::Q, key_down),
	    keyboard::Keycode::R =>
		key_states.set(key_states::Keycode::R, key_down),
	    keyboard::Keycode::S =>
		key_states.set(key_states::Keycode::S, key_down),
	    keyboard::Keycode::W =>
		key_states.set(key_states::Keycode::W, key_down),
	    keyboard::Keycode::Return =>
		key_states.set(key_states::Keycode::Return, key_down),	    
	    keyboard::Keycode::Space =>
		key_states.set(key_states::Keycode::Space, key_down),
	    keyboard::Keycode::Escape =>
		key_states.set(key_states::Keycode::Escape, key_down),
	    keyboard::Keycode::LShift =>
		key_states.set(key_states::Keycode::LShift, key_down),
	    keyboard::Keycode::LCtrl =>
		key_states.set(key_states::Keycode::LCtrl, key_down),
	    _ => (),
	}
    }
}

pub enum InputState {
    Continue,
    Pause,
    Quit,
    Reset,
    Tick,
}
