use bevy::{app::AppExit, prelude::*, window::CursorGrabMode};

use crate::{
    components::{
        fade_transition::{transition_to, TransitionState},
        ui_util::{btn, txt},
    },
    game::keybinds::FloraCommand,
    AppState,
};

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseState>()
            .add_systems(OnEnter(AppState::Paused), setup)
            .add_systems(OnExit(AppState::Paused), teardown)
            .add_systems(
                Update,
                (toggle_pause, interact).run_if(in_state(AppState::Paused)),
            );
    }
}

#[derive(Resource, Default)]
pub struct PauseState {
    pub paused: bool,
    pub previous_state: Option<AppState>,
}

pub fn toggle_pause(
    input: Res<Input<FloraCommand>>,
    mut state: ResMut<PauseState>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(FloraCommand::Pause) {
        state.paused = !state.paused;
        if state.paused {
            state.previous_state = Some(*app_state.get());
            next_state.set(AppState::Paused);
        } else {
            if let Some(next) = &state.previous_state {
                next_state.set(*next);
            }
            state.previous_state = None;
        }
    }
}

pub fn capture_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn release_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::None;
}

#[derive(Component)]
enum Action {
    ReturnToMenu,
    Unpause,
    Volume,
}

/// Marker component for anything on the Main Menu screen.
/// Used for despawning all UI nodes when leaving Main Menu screen
#[derive(Component)]
struct OnMainMenuScreen;

/// React to button presses
fn interact(
    interaction_query: Query<(&Interaction, &Action), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<PauseState>,
    mut transition_state: ResMut<TransitionState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                Action::ReturnToMenu => {
                    transition_to(AppState::MainMenu, &mut transition_state);
                    state.previous_state = None;
                }
                Action::Unpause => {
                    if let Some(next) = &state.previous_state {
                        next_state.set(*next);
                    }
                    state.previous_state = None;
                }
                _ => todo!("Handle volume controls"),
            }
        }
    }
}

/// Runs when we enter [AppState::MainMenu]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = txt(&mut commands, "Game Paused!", 48.0, &asset_server);
    let return_btn = btn(
        &mut commands,
        "Return to Menu",
        Action::ReturnToMenu,
        &asset_server,
    );
    let unpause_btn = btn(&mut commands, "Unpause", Action::Unpause, &asset_server);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.4).into(),
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
            cb.add_child(title);
            cb.add_child(return_btn);
            cb.add_child(unpause_btn);
        });
}

/// Runs when we exit [AppState::MainMenu]
fn teardown(nodes: Query<Entity, With<OnMainMenuScreen>>, mut commands: Commands) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}
