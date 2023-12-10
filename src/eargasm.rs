use bevy::audio::{AudioBundle, AudioSink, AudioSource};
use bevy::ecs::system::SystemParam;
use bevy::{log, prelude::*};

pub struct EargasmPlugin;
impl Plugin for EargasmPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding Eargasm plugin");
        app.add_event::<AudioRequest>()
            .add_systems(Startup, setup)
            .add_systems(Update, (play_system, once_fire_music_test));

        info!("Added Eargasm plugin");
    }
}

fn once_fire_music_test(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AudioRequest>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        info!("Starting music");
        event_writer.send(AudioRequest {
            component: AudioComponent::Track1(Track1),
        });
        info!("Sent event!");
    }
}

#[derive(Component)]
pub enum AudioComponent {
    Track1(Track1),
    Track2(Track2),
    Radar1(Radar1),
    Radar2(Radar2),
    TheCompanyThanksYou(TheCompanyThanksYou),
    IntroVoice(IntroVoice),
}

#[derive(Component)]
pub struct Track1;
#[derive(Component)]
pub struct Track2;
#[derive(Component)]
pub struct Radar1;
#[derive(Component)]
pub struct Radar2;
#[derive(Component)]
pub struct TheCompanyThanksYou;
#[derive(Component)]
pub struct IntroVoice;

#[derive(Event)]
pub struct AudioRequest {
    pub component: AudioComponent,
}

#[derive(SystemParam)]
pub struct AudioSysParam<'w, 's> {
    // Use queries without explicit reference lifetimes
    query_track1: Query<'w, 's, (&'static AudioSink, &'static Handle<AudioSource>), With<Track1>>,
    query_track2: Query<'w, 's, (&'static AudioSink, &'static Handle<AudioSource>), With<Track2>>,
    asset_server: Res<'w, AssetServer>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let audio_assets = [
        ("audio/musictrack 1.mp3", AudioComponent::Track1(Track1)),
        ("audio/musictrack 2.mp3", AudioComponent::Track2(Track2)),
        // ... other assets ...
    ];

    for (path, component) in audio_assets {
        let audio_source = asset_server.load(path);
        commands
            .spawn(AudioBundle {
                source: audio_source,
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    // volume: todo!(),
                    // speed: todo!(),
                    // paused: todo!(),
                    // spatial: todo!(),
                    ..default()
                },
            })
            .insert(component);
    }
}

fn play_system(param: AudioSysParam, mut event_reader: EventReader<AudioRequest>) {
    for event in event_reader.read() {
        info!("Read AudioRequest");
        match &event.component {
            AudioComponent::Track1(_) => {
                let q = param.query_track1.get_single().unwrap();
                q.0.toggle();
                info!("Toggled");
            }
            _ => todo!(" Add the other tracks n audio"),
        };
    }
}
