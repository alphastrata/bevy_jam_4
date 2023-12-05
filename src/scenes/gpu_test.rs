use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct DevScenePlugin;
impl Plugin for DevScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), enter_menu)
            .add_systems(OnExit(AppState::MainMenu), exit_menu)
            .add_systems(
                Update,
                (button_system, menu_action).run_if(in_state(AppState::MainMenu)),
            );
    }
}

#[derive(Component)]
enum MenuButtonAction {
    StartGame,
    TestScene,
    QuitGame,
    Credits,
    SetVolume,
}

/// Marker component for anything on the Main Menu screen.
/// Used for despawning all UI nodes when leaving Main Menu screen
#[derive(Component)]
struct OnMainMenuScreen;

mod button_styles {
    use bevy::prelude::Color;

    pub(super) const NORMAL: Color = Color::rgb(0.15, 0.15, 0.15);
    pub(super) const HOVERED: Color = Color::rgb(0.30, 0.50, 0.30);
    pub(super) const PRESSED: Color = Color::rgb(0.35, 0.35, 0.35);
}

/// React to button presses
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::StartGame => {
                    app_state.set(AppState::Playing);
                }
                // the game can't quit in browser lmao
                MenuButtonAction::QuitGame => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Credits => {
                    // show credits!
                }
                _ => todo!("Handle volume controls"),
            }
        }
    }
}

/// Runs when we enter [AppState::MainMenu]
fn enter_menu(mut commands: Commands) {
    let start_button = spawn_button(&mut commands, "Start Game");
    commands
        .entity(start_button)
        .insert(MenuButtonAction::StartGame);

    let gpu_test = spawn_button(&mut commands, "Test Scene");
    commands
        .entity(gpu_test)
        .insert(MenuButtonAction::TestScene);

    #[cfg(not(target_arch = "wasm32"))]
    let quit_button = spawn_button(&mut commands, "Quit Game");
    #[cfg(not(target_arch = "wasm32"))]
    commands
        .entity(quit_button)
        .insert(MenuButtonAction::QuitGame);

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
            cb.add_child(start_button);
            cb.add_child(gpu_test);

            #[cfg(not(target_arch = "wasm32"))]
            {
                cb.add_child(quit_button);
            }
        });
}

/// Runs when we exit [AppState::MainMenu]
fn exit_menu(nodes: Query<Entity, With<OnMainMenuScreen>>, mut commands: Commands) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}

/// Spawn a generic button
fn spawn_button(commands: &mut Commands, text: &str) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(100.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: button_styles::NORMAL.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    // TODO: load font - font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ));
        })
        .id()
}

/// Handles changing the button styles
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = button_styles::PRESSED.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = button_styles::HOVERED.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = button_styles::NORMAL.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
