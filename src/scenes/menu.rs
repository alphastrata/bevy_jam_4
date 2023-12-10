use std::f32::consts::PI;

use bevy::{
    app::AppExit, core_pipeline::clear_color::ClearColorConfig, prelude::*,
    render::render_resource::Extent3d,
};

use crate::{
    components::{
        fade_transition::{transition_to, TransitionState},
        ui_util::{btn, img, txt, GameFont},
    },
    creeps::initial_creep_spawn,
    game::camera::{main_layer, rt_cam3d, v3d_layer, UiCamera},
    AppState,
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(OnExit(AppState::MainMenu), teardown)
            .add_systems(
                Update,
                (interact, rotator_system).run_if(in_state(AppState::MainMenu)),
            );
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
#[derive(Component, Default)]
struct OnMainMenuScreen;
// impl Default for OnMainMenuScreen {
//     fn default() -> Self {}
// }

#[derive(Component, Default)]
struct DemoCube;

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
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    font: Res<GameFont>,
    asset_server: Res<AssetServer>,
) {
    let (img_handle, camera) = rt_cam3d(
        &mut commands,
        &mut images,
        Extent3d {
            width: 120,
            height: 80,
            ..default()
        },
        v3d_layer(),
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            camera: Camera {
                order: -1,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    );
    commands.entity(camera).insert(OnMainMenuScreen);

    let ui_image = commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    ..default()
                },
                image: UiImage::new(img_handle.clone()),
                z_index: ZIndex::Global(i32::MIN),
                ..default()
            },
            main_layer(),
            OnMainMenuScreen,
        ))
        .id();

    let cube_size = 4.0;
    let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // Main pass cube, with material containing the rendered first pass texture.
    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.5)
                .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
            ..default()
        },
        v3d_layer(),
        OnMainMenuScreen,
        DemoCube,
    ));

    commands.spawn((
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        v3d_layer(),
        OnMainMenuScreen,
    ));

    let title = img(
        &mut commands,
        asset_server.load("textures/title.png"),
        Some(Val::Px(512.0)),
        None,
    );
    let start_button = btn(&mut commands, &font, "Start Game", Action::StartGame);
    let gpu_test = btn(&mut commands, &font, "Dev Scene", Action::DevScene);

    #[cfg(not(target_arch = "wasm32"))]
    let quit_button = btn(&mut commands, &font, "Quit Game", Action::QuitGame);

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

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<DemoCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}

/// Runs when we exit [AppState::MainMenu]
fn teardown(mut commands: Commands, nodes: Query<Entity, With<OnMainMenuScreen>>) {
    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}
