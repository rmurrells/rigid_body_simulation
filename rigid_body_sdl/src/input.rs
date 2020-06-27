use crate::StrResult;
use rigid_body_core::input::{
    InputCore,
    InputEvent,
    keyboard_state,
    mouse_state,
};
use sdl2::{
    event::Event,
    EventPump,
    keyboard,
    mouse,
    Sdl,
};

pub struct InputSDL {
    event_pump: EventPump,
}

impl InputSDL {
    pub fn new(context: &Sdl) -> StrResult<Self> {
        Ok(Self {
	    event_pump: context.event_pump()?,
	})
    }
    
    pub fn get(&mut self, input_core: &mut InputCore) {
	for event in self.event_pump.poll_iter() {
	    if let Some(input_event) = match event {
		Event::KeyDown{keycode: Some(key), ..} =>
		    Self::match_key(key).map(|key| InputEvent::KeyDown{key}),
		Event::KeyUp{keycode: Some(key), ..} =>
		    Self::match_key(key).map(|key| InputEvent::KeyUp{key}),
		Event::MouseButtonDown{mouse_btn, ..} =>
		    Self::match_button(mouse_btn)
		    .map(|button| InputEvent::MouseButtonDown{button}),
		Event::MouseButtonUp{mouse_btn, ..} =>
		    Self::match_button(mouse_btn)
		    .map(|button| InputEvent::MouseButtonUp{button}),
		Event::MouseMotion{x, y, xrel, yrel, ..} =>
		    Some(InputEvent::MouseMotion{x, y, xrel, yrel}),
		Event::MouseWheel{x, y, ..} =>
		    Some(InputEvent::MouseWheel{x, y}),
		Event::Quit{..} => Some(InputEvent::Quit),
		_ => None,
	    } {
		input_core.handle_event(input_event);
	    }
	}
    }

    fn match_button(
	button: mouse::MouseButton,
    ) -> Option<mouse_state::MouseButton> {
	match button {
	    mouse::MouseButton::Left => Some(mouse_state::MouseButton::Left),
	    mouse::MouseButton::Middle => Some(mouse_state::MouseButton::Middle),
	    mouse::MouseButton::Right => Some(mouse_state::MouseButton::Right),
	    _ => None,
	}
    }
    
    fn match_key(key: keyboard::Keycode) -> Option<keyboard_state::Keycode> {
	match key {
	    keyboard::Keycode::A => Some(keyboard_state::Keycode::A),
	    keyboard::Keycode::D => Some(keyboard_state::Keycode::D),
	    keyboard::Keycode::E => Some(keyboard_state::Keycode::E),
	    keyboard::Keycode::Q => Some(keyboard_state::Keycode::Q),
	    keyboard::Keycode::R => Some(keyboard_state::Keycode::R),
	    keyboard::Keycode::S => Some(keyboard_state::Keycode::S),
	    keyboard::Keycode::W => Some(keyboard_state::Keycode::W),
	    keyboard::Keycode::Return => Some(keyboard_state::Keycode::Return),
	    keyboard::Keycode::Space => Some(keyboard_state::Keycode::Space),
	    keyboard::Keycode::Escape => Some(keyboard_state::Keycode::Escape),
	    _ => None,
	}
    }
}
