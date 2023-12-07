use crate::prelude::*;
use bevy::{asset::processor::ProcessorTransactionLog, ecs::bundle, prelude::*};
use rand::Rng;

pub struct CreepPlugin;
impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.insert_resource(CreepCount(0));

        app.add_event::<SpawnCreep>();

        app.add_systems(Update, (sprite_movement, spawn_creep));

        #[cfg(debug_assertions)]
        app.add_systems(Update, (dbg_send_spawn_creep_on_enter, dbg_count_creeps));
    }
}

#[derive(Component)]
pub struct Creep;

#[derive(Event)]
pub struct SpawnCreep;

#[cfg(debug_assertions)]
#[derive(Resource)]
pub struct CreepCount(pub usize);

#[cfg(debug_assertions)]
fn dbg_count_creeps(q: Query<&Transform, With<Creep>>, mut count: ResMut<CreepCount>) {
    (*count) = CreepCount(q.iter().count());
}

#[cfg(debug_assertions)]
fn dbg_send_spawn_creep_on_enter(mut spawner: EventWriter<SpawnCreep>, kb: Res<Input<KeyCode>>) {
    if kb.just_released(KeyCode::Return) {
        spawner.send(SpawnCreep);
        println!("Creep will spawn!");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn spawn_creep(
    mut spawner: EventReader<SpawnCreep>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //TODO: random xy spawn at distance from center of map...
    spawner.read().for_each(|_spawn_event| {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-0.5..=0.5);
        let y = rng.gen_range(-0.5..=0.5);
        //TODO: Keep a creep count, resource for UI?
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.10),
                texture: asset_server.load("textures/creep.png"),
                //
                ..default()
            },
            Creep,
            //TODO: creep stats should probs be set by .csv or something?
            MovementSpeed(10),
            AttackSpeed(10),
            Health(100),
            Experience(100),
            Heading::Stationary,
        ));
        return;
    })
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Heading, &mut Transform)>) {
    for (mut creep, mut transform) in &mut sprite_position {
        match *creep {
            Heading::North => transform.translation.y += 150. * time.delta_seconds(),
            Heading::South => transform.translation.y -= 150. * time.delta_seconds(),
            Heading::East => transform.translation.x -= 150. * time.delta_seconds(),
            Heading::West => transform.translation.x += 150. * time.delta_seconds(),
            _ => return,
        }

        if transform.translation.y > 200. {
            *creep = Heading::South;
        } else if transform.translation.y < -200. {
            *creep = Heading::North;
        }
    }
}
