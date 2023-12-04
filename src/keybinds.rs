use bevy::{
    app::{App, Plugin, Update},
    ecs::event::EventReader,
    input::keyboard::KeyboardInput,
    log::info,
};

pub struct KeybindPlugin;
impl Plugin for KeybindPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_events);
    }
}

fn keyboard_events(mut ev: EventReader<KeyboardInput>) {
    for event in ev.read() {
        info!("{:?}", event);
    }
}
