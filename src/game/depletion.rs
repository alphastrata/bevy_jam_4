use bevy::prelude::*;

use crate::{buildings::Building, AppState, GameOver};

use super::resources::Inventory;

#[derive(Resource)]
struct DepleteTick(Timer);

pub struct DepletionPlugin;
impl Plugin for DepletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .insert_resource(DepleteTick(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(
                Update,
                (deplete_your_bank_account).run_if(in_state(AppState::Gameplay)),
            );
    }
}

fn deplete_your_bank_account(
    mut timer: ResMut<DepleteTick>,
    mut inventory: ResMut<Inventory>,
    mut game_over: EventWriter<GameOver>,
    q_all_buildings: Query<(Entity, &Building)>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        info!("Depleting player's corpo points");
        let multiplier = time_multiplier(time.elapsed_seconds() as u32);
        let num_buildings = q_all_buildings.iter().count();
        let to_subtract = multiplier * num_buildings as f32;

        if to_subtract as u32 > inventory.money {
            game_over.send(GameOver);
            info!("GAME OVER");
        } else {
            inventory.money -= to_subtract as u32;
        }
    }
}

fn time_multiplier(seconds: u32) -> f32 {
    match seconds {
        // 0..=60 => 1.0,
        // 61..=120 => 2.0,
        // 121..=240 => 2.5,
        // _x => 3.5,
        0..=60 => 1.0,
        61..=120 => 1.5,
        121..=240 => 2.5,
        _x => 4.0,
    }
}
