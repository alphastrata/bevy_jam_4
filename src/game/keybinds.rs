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
    Pause,
    Left,
    Right,
    Up,
    Down,
    Copy,
    Paste,
    Debug,
    ResetCamera,
    SetPlaceDistributionTower,
    SetPlaceRadarTower,
}

lazy_static! {
    static ref FLORA_COMMAND_MAPPING: HashMap<FloraCommand, Vec<Vec<KeyCode>>> = {
        let mut map = HashMap::new();
        map.insert(FloraCommand::Pause, vec![vec![KeyCode::Escape]]);
        map.insert(FloraCommand::Left, vec![vec![KeyCode::Left]]);
        map.insert(FloraCommand::Right, vec![vec![KeyCode::Right]]);
        map.insert(FloraCommand::Up, vec![vec![KeyCode::Up]]);
        map.insert(FloraCommand::Down, vec![vec![KeyCode::Down]]);
        map.insert(FloraCommand::Debug, vec![vec![KeyCode::F12]]);
        map.insert(FloraCommand::ResetCamera, vec![vec![KeyCode::F5]]);
        map.insert(
            FloraCommand::Copy,
            vec![
                vec![KeyCode::ControlLeft, KeyCode::C],
                vec![KeyCode::ControlRight, KeyCode::C],
            ],
        );
        map.insert(
            FloraCommand::Paste,
            vec![
                vec![KeyCode::ControlLeft, KeyCode::V],
                vec![KeyCode::ControlRight, KeyCode::V],
            ],
        );
        // Tower selection
        map.insert(FloraCommand::SetPlaceDistributionTower, vec![vec![KeyCode::Key1]]);
        map.insert(FloraCommand::SetPlaceRadarTower, vec![vec![KeyCode::Key2]]);

        map
    };
}

fn keyboard_events(input: Res<Input<KeyCode>>, mut resource: ResMut<Input<FloraCommand>>) {
    for flora in FLORA_COMMAND_MAPPING.keys() {
        let combos = &FLORA_COMMAND_MAPPING[flora];

        let mut pressed = false;
        for combo in combos {
            let mut combo_pressed = true;
            for key in combo {
                // quit out if any keys are not pressed
                if !input.pressed(*key) {
                    combo_pressed = false;
                    break;
                }
            }
            if combo_pressed {
                pressed = true;
                break;
            }
        }

        if pressed {
            if resource.pressed(*flora) {
                resource.clear_just_pressed(*flora);
            }
            resource.press(*flora);
        } else if resource.pressed(*flora) {
            resource.release(*flora);
        } else {
            resource.reset(*flora);
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
