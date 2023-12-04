use bevy::{
	app::{App, Plugin, PreUpdate, Update},
	ecs::{
		event::{Event, EventReader, EventWriter},
		system::{Res, ResMut},
	},
	input::{
		keyboard::{KeyCode, KeyboardInput},
		Input,
	},
	log::info,
	reflect::Reflect,
	time::Time,
};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct KeybindPlugin;
impl Plugin for KeybindPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Input<FloraCommand>>()
			.add_systems(PreUpdate, keyboard_events)
			.add_systems(Update, something);
	}
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
#[cfg_attr(
	feature = "serialize",
	derive(serde::Serialize, serde::Deserialize),
	reflect(Serialize, Deserialize)
)]
#[repr(u32)]
pub enum FloraCommand {
	Esc,
	Left,
	Right,
	Up,
	Down,
	Copy,
	Paste,
}

lazy_static! {
	static ref FLORA_COMMAND_MAPPING: HashMap<Vec<KeyCode>, FloraCommand> = {
		let mut map = HashMap::new();
		map.insert(vec![KeyCode::Escape], FloraCommand::Esc);
		map.insert(vec![KeyCode::Left], FloraCommand::Left);
		map.insert(vec![KeyCode::Right], FloraCommand::Right);
		map.insert(vec![KeyCode::Up], FloraCommand::Up);
		map.insert(vec![KeyCode::Down], FloraCommand::Down);
		map.insert(vec![KeyCode::ControlLeft, KeyCode::C], FloraCommand::Copy);
		map.insert(vec![KeyCode::ControlLeft, KeyCode::V], FloraCommand::Paste);
		map
	};
}

fn keyboard_events(input: Res<Input<KeyCode>>, mut resource: ResMut<Input<FloraCommand>>) {
	for combo in FLORA_COMMAND_MAPPING.keys() {
		let flora = FLORA_COMMAND_MAPPING[combo];

		let mut pressed = true;
		for key in combo {
			// quit out if any keys are not pressed
			if !input.pressed(*key) {
				pressed = false;
				break;
			}
		}
		if pressed {
			if resource.pressed(flora) {
				resource.clear_just_pressed(flora);
			}
			resource.press(flora);
		} else {
			if resource.pressed(flora) {
				resource.release(flora);
			} else {
				resource.reset(flora);
			}
		}
	}
	// info!("{:?}", event);
}

fn something(inputs: Res<Input<FloraCommand>>, time: Res<Time>) {
	if inputs.just_pressed(FloraCommand::Copy) {
		info!("Copy at {}", time.elapsed_seconds());
	}
	if inputs.pressed(FloraCommand::Copy) {
		info!("Copying at {}", time.elapsed_seconds());
	}
	if inputs.just_released(FloraCommand::Copy) {
		info!("Copyed at {}", time.elapsed_seconds());
	}
	if inputs.just_pressed(FloraCommand::Paste) {
		info!("Paste at {}", time.elapsed_seconds());
	}
}
