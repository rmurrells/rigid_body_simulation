use rigid_body_core::input::{
    InputCore,
    InputEvent,
    keyboard_state,
    mouse_state,
};

pub fn key(key: u32, down: bool, input_core: &mut InputCore) {
    if let Some(key) = match key {
	65 => Some(keyboard_state::Keycode::A),
        68 => Some(keyboard_state::Keycode::D),
        69 => Some(keyboard_state::Keycode::E),
        81 => Some(keyboard_state::Keycode::Q),
        82 => Some(keyboard_state::Keycode::R),
        83 => Some(keyboard_state::Keycode::S),
        87 => Some(keyboard_state::Keycode::W),
        13 => Some(keyboard_state::Keycode::Return),
	32 => Some(keyboard_state::Keycode::Space),
        27 => Some(keyboard_state::Keycode::Escape),
	_ => None,
    } {
	input_core.handle_event(
	    if down {InputEvent::KeyDown{key}}
	    else {InputEvent::KeyUp{key}}
	);
    }
}

pub fn mouse_button(button: u32, down: bool, input_core: &mut InputCore) {
    if let Some(button) = match button {
	0 => Some(mouse_state::MouseButton::Left),
	_ => None,
    } {
	input_core.handle_event(
	    if down {InputEvent::MouseButtonDown{button}}
	    else {InputEvent::MouseButtonUp{button}}
	);	
    }
}

pub fn mouse_move(x: i32, y: i32, input_core: &mut InputCore) {
    let mouse_state = &input_core.mouse_state;
    let xrel = x-mouse_state.x;
    let yrel = y-mouse_state.y;
    input_core.handle_event(InputEvent::MouseMotion{x, y, xrel, yrel});
}

pub fn mouse_wheel(xrel: i32, yrel: i32, input_core: &mut InputCore) {
    input_core.handle_event(InputEvent::MouseWheel{xrel, yrel: -yrel});
}
