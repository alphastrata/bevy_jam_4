use bevy::audio::{AudioBundle, AudioSink, AudioSource, PlaybackMode, Volume, VolumeLevel};
use bevy::ecs::system::SystemParam;
use bevy::{log, prelude::*};

pub struct EargasmPlugin;
impl Plugin for EargasmPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioRequest>()
            .add_systems(Startup, setup)
            .add_systems(Update, play_system);

        debug!("Added Eargasm plugin");
    }
}

#[derive(Component, Debug)]
pub enum AudioComponent {
    Track1(Track1),
    Track2(Track2),
    Thump(Thump),
    Radar1(Radar1),
    Radar2(Radar2),
    TheCompanyThanksYou(TheCompanyThanksYou),
    IntroVoice(IntroVoice),
    Electric(Electric),
    Money(Money),
}

#[derive(Component, Debug)]
pub struct Money;
#[derive(Component, Debug)]
pub struct Thump;
#[derive(Component, Debug)]
pub struct Track1;
#[derive(Component, Debug)]
pub struct Track2;
#[derive(Component, Debug)]
pub struct Radar1;
#[derive(Component, Debug)]
pub struct Radar2;
#[derive(Component, Debug)]
pub struct TheCompanyThanksYou;
#[derive(Component, Debug)]
pub struct IntroVoice;
#[derive(Component, Debug)]
pub struct Electric;

/// Event:
/// This is the main thing you use when you want to trigger some audio.
#[derive(Event)]
pub struct AudioRequest {
    pub component: AudioComponent,
}

/// System: Startup -> inserts & plays the title track with: PlaybackMode::Once, so it will NOT loop.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(AudioBundle {
            source: asset_server.load("audio/musictrack 1.mp3"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                paused: false,
                ..default()
            },
        })
        .insert(Track1);
}

/// System: plays sounds from our `assets/audio/*.mp3`, each option there is named for an Event that you can fire with the :
/// EventWriter<[`AudioRequest`]>, which takes an [`AudioComponent`]
fn play_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<AudioRequest>,
) {
    let vol = Volume::Relative(VolumeLevel::new(4.0));

    for event in event_reader.read() {
        //TODO: if track1 or 2 is already playing -- we want to stop that? (if on track1 and request == 2 and vice versa...)
        info!("Read AudioRequest");
        match &event.component {
            // SFX
            AudioComponent::Thump(_t) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/thump.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: vol,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Thump);
            }
            AudioComponent::Money(_m) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/money.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: vol,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Money);
            }
            AudioComponent::Electric(_e) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/electric.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: vol,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Electric);
            }

            AudioComponent::Radar1(_) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/radar1.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: vol,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Radar1);
            }
            AudioComponent::Radar2(_) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/radar2.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: vol,
                            // Add any other custom settings if needed
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Radar2);
            }
            AudioComponent::TheCompanyThanksYou(_) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/thecompanythanksyou.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: vol,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(TheCompanyThanksYou);
            }
            AudioComponent::IntroVoice(_) => {
                // Similar implementation for IntroVoice
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/introductionvoice.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: Volume::Absolute(VolumeLevel::new(0.05)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(IntroVoice);
            }

            //MUSIC:
            AudioComponent::Track1(_) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/musictrack 1.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            paused: false,
                            volume: vol,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track1);
            }
            AudioComponent::Track2(_) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/musictrack 2.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Loop,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track2);
            }

            #[allow(unreachable_code)]
            _ => error!("Unknown audiosink/source/track pairing!"),
        }
    }
}
