use bevy::{app::AppExit, prelude::*, window::CursorGrabMode};

use crate::{
    game::keybinds::FloraCommand,
    global_systems::{
        fade_transition::{transition_to, TransitionState},
        ui_util::{btn, txt, GameFont},
    },
    AppState, PauseMenuState,
};

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PauseMenuState::Paused), (setup, release_cursor))
            .add_systems(OnExit(PauseMenuState::Paused), (teardown, capture_cursor))
            .add_systems(
                Update,
                (interact, check_for_keyboard_unpause).run_if(in_state(PauseMenuState::Paused)),
            );
    }
}

pub fn check_for_keyboard_pause(
    input: Res<Input<FloraCommand>>,
    mut next_state: ResMut<NextState<PauseMenuState>>,
) {
    if input.just_pressed(FloraCommand::Pause) {
        next_state.set(PauseMenuState::Paused);
    }
}

pub fn check_for_keyboard_unpause(
    input: Res<Input<FloraCommand>>,
    mut next_state: ResMut<NextState<PauseMenuState>>,
) {
    if input.just_pressed(FloraCommand::Pause) {
        info!("unpause");
        next_state.set(PauseMenuState::Unpaused);
    }
}

pub fn capture_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Confined;
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
    mut transition_state: ResMut<TransitionState>,
    mut next_state_pause: ResMut<NextState<PauseMenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                Action::ReturnToMenu => {
                    transition_to(AppState::MainMenu, &mut transition_state);
                }
                Action::Unpause => next_state_pause.set(PauseMenuState::Unpaused),
                _ => todo!("Handle volume controls"),
            }
        }
    }
}

/// Runs when we enter [AppState::MainMenu]
fn setup(mut commands: Commands, font: Res<GameFont>) {
    info!("am pause");
    let title = txt(&mut commands, &font, "Game Paused!", 64.0);
    let return_btn = btn(&mut commands, &font, "Return to Menu", Action::ReturnToMenu);
    let unpause_btn = btn(&mut commands, &font, "Unpause", Action::Unpause);

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
            cb.add_child(unpause_btn);
            cb.add_child(return_btn);
        });
}

/// Runs when we exit [AppState::MainMenu]
fn teardown(nodes: Query<Entity, With<OnMainMenuScreen>>, mut commands: Commands) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}
