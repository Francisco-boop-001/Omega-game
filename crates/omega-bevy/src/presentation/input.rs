use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
use bevy::prelude::*;

use crate::{BevyKey, FrontendRuntime};

pub fn keyboard_to_runtime_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut events: EventReader<KeyboardInput>,
    mut runtime: ResMut<FrontendRuntime>,
) {
    for event in events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        if let Some(key) = map_key(event, &keys) {
            runtime.0.handle_key(key);
        }
    }
}

fn map_key(event: &KeyboardInput, keys: &ButtonInput<KeyCode>) -> Option<BevyKey> {
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    match &event.logical_key {
        Key::ArrowUp => Some(BevyKey::Up),
        Key::ArrowDown => Some(BevyKey::Down),
        Key::ArrowLeft => Some(BevyKey::Left),
        Key::ArrowRight => Some(BevyKey::Right),
        Key::Escape => Some(BevyKey::Esc),
        Key::Enter => Some(BevyKey::Enter),
        Key::Backspace => Some(BevyKey::Backspace),
        Key::F8 => Some(BevyKey::F8),
        Key::F12 => Some(BevyKey::F12),
        Key::Character(text) => {
            let ch = text.chars().next()?;
            if ch.is_ascii_control() {
                return None;
            }
            if ctrl && ch.is_ascii_alphabetic() {
                return Some(BevyKey::Ctrl(ch.to_ascii_lowercase()));
            }
            Some(BevyKey::Char(ch))
        }
        _ => match event.key_code {
            KeyCode::ArrowUp => Some(BevyKey::Up),
            KeyCode::ArrowDown => Some(BevyKey::Down),
            KeyCode::ArrowLeft => Some(BevyKey::Left),
            KeyCode::ArrowRight => Some(BevyKey::Right),
            KeyCode::Escape => Some(BevyKey::Esc),
            KeyCode::Enter => Some(BevyKey::Enter),
            KeyCode::Backspace => Some(BevyKey::Backspace),
            KeyCode::F8 => Some(BevyKey::F8),
            KeyCode::F12 => Some(BevyKey::F12),
            _ => None,
        },
    }
}
