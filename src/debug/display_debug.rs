use bevy::prelude::*;

use crate::game::keybinds::FloraCommand;

use super::{fps_counter::FPSPlugin, resolution::ResolutionPlugin};

#[derive(Resource, Default)]
pub struct DebugState {
    pub is_debug: bool,
}

pub struct DisplayDebugPlugin;
impl Plugin for DisplayDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<DebugState>()
            .add_plugins((FPSPlugin, ResolutionPlugin))
            .add_systems(Update, set_debug);
    }
}

fn set_debug(mut state: ResMut<DebugState>, input: Res<Input<FloraCommand>>) {
    if input.just_pressed(FloraCommand::Debug) {
        state.is_debug = !state.is_debug;
    }
}

/// Toggle the FPS counter when pressing F12
pub fn toggle_visibility<T: Component>(
    mut q: Query<&mut Visibility, With<T>>,
    debug: Res<DebugState>,
) {
    let mut vis = q.single_mut();
    *vis = if debug.is_debug {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}
