use bevy::{app::AppExit, prelude::*};

use crate::{
    components::{
        fade_transition::{transition_to, TransitionState},
        ui_util::{btn, txt},
    },
    AppState,
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(OnExit(AppState::MainMenu), teardown)
            .add_systems(Update, interact.run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(Component)]
enum Action {
    StartGame,
    DevScene,
    QuitGame,
    Credits,
    SetVolume,
}

/// Marker component for anything on the Main Menu screen.
/// Used for despawning all UI nodes when leaving Main Menu screen
#[derive(Component)]
struct OnMainMenuScreen;

/// React to button presses
fn interact(
    interaction_query: Query<(&Interaction, &Action), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut transition_state: ResMut<TransitionState>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                Action::StartGame => {
                    transition_to(AppState::Gameplay, &mut transition_state);
                }
                Action::DevScene => {
                    app_state.set(AppState::DevScene);
                }
                // the game can't quit in browser lmao
                Action::QuitGame => {
                    app_exit_events.send(AppExit);
                }
                Action::Credits => {
                    // show credits!
                }
                _ => todo!("Handle volume controls"),
            }
        }
    }
}

/// Runs when we enter [AppState::MainMenu]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = txt(&mut commands, "Flora Cause", 40.0, &asset_server);
    let start_button = btn(
        &mut commands,
        "Start Game",
        Action::StartGame,
        &asset_server,
    );
    let gpu_test = btn(&mut commands, "Dev Scene", Action::DevScene, &asset_server);

    #[cfg(not(target_arch = "wasm32"))]
    let quit_button = btn(&mut commands, "Quit Game", Action::QuitGame, &asset_server);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            let mut cb = parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
            cb.add_child(title)
                .add_child(start_button)
                .add_child(gpu_test);

            #[cfg(not(target_arch = "wasm32"))]
            {
                cb.add_child(quit_button);
            }
        });
}

/// Runs when we exit [AppState::MainMenu]
fn teardown(mut commands: Commands, nodes: Query<Entity, With<OnMainMenuScreen>>) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}
