use bevy::{app::AppExit, prelude::*, window::CursorGrabMode};

use crate::{
    game::keybinds::FloraCommand,
    global_systems::{
        fade_transition::{transition_to, TransitionState},
        ui_util::{btn, img, txt, GameFont},
    },
    AppState, PauseMenuState,
};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), (setup, release_cursor))
            .add_systems(OnExit(AppState::GameOver), (teardown, capture_cursor))
            .add_systems(Update, (interact).run_if(in_state(AppState::GameOver)));
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
}

#[derive(Component)]
struct OnGameOverScreen;

/// React to button presses
fn interact(
    interaction_query: Query<(&Interaction, &Action), (Changed<Interaction>, With<Button>)>,
    mut transition_state: ResMut<TransitionState>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                Action::ReturnToMenu => {
                    transition_to(AppState::MainMenu, &mut transition_state);
                }
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, font: Res<GameFont>) {
    info!("am dead");
    let tex = asset_server.load("textures/fired.png");
    let title = txt(&mut commands, &font, "GAME OVER", 64.0);
    let fired = img(
        &mut commands,
        tex,
        Some(Val::Px(117.0 * 3.0)),
        Some(Val::Px(75.0 * 3.0)),
    );
    let return_btn = btn(
        &mut commands,
        &font,
        "Return to Main Menu",
        Action::ReturnToMenu,
    );

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
            OnGameOverScreen,
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
            cb.add_child(fired);
            cb.add_child(return_btn);
        });
}

fn teardown(nodes: Query<Entity, With<OnGameOverScreen>>, mut commands: Commands) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}
