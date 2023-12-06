use bevy::{app::AppExit, prelude::*};

use crate::{components::button::spawn_button, AppState};

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
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                Action::StartGame => {
                    app_state.set(AppState::Gameplay);
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
fn setup(mut commands: Commands) {
    let start_button = spawn_button(&mut commands, "Start Game", Action::StartGame);

    let gpu_test = spawn_button(&mut commands, "Dev Scene", Action::DevScene);

    #[cfg(not(target_arch = "wasm32"))]
    let quit_button = spawn_button(&mut commands, "Quit Game", Action::QuitGame);

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
                background_color: Color::CRIMSON.into(),
                ..default()
            });
            cb.add_child(start_button).add_child(gpu_test);

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
