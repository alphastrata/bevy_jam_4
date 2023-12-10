use bevy::prelude::*;

use crate::{buildings::BuildingType, AppState};

use super::placement::{PlacementState, PlacementStateChanged};

#[derive(Component, Default)]
struct HudElement;

const PIXEL: f32 = 4.0;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), setup)
            .add_systems(OnExit(AppState::Gameplay), teardown)
            .add_systems(
                Update,
                (interact, set_button_state).run_if(in_state(AppState::Gameplay)),
            );
    }
}

pub mod btn_styles {
    pub(super) const NORMAL: usize = 0;
    pub(super) const HOVERED: usize = 1;
    pub(super) const PRESSED: usize = 2;
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // background
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    align_items: AlignItems::End,
                    ..default()
                },
                z_index: ZIndex::Global(i32::MAX - 1),
                ..default()
            },
            HudElement,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Vw(100.0),
                        height: Val::Px(PIXEL * 26.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(asset_server.load("textures/ui-bar-left.png")),
                        ..default()
                    });
                    parent.spawn(ImageBundle {
                        image: UiImage::new(asset_server.load("textures/ui-bar-mid.png")),
                        style: Style {
                            flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load("textures/ui-bar-mid-screen-left.png"),
                        ),
                        ..default()
                    });
                    parent.spawn(ImageBundle {
                        image: UiImage::new(asset_server.load("textures/ui-bar-mid-screen.png")),
                        ..default()
                    });
                    parent.spawn(ImageBundle {
                        image: UiImage::new(asset_server.load("textures/ui-bar-right.png")),
                        ..default()
                    });
                });
        });

    let drain_handle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("textures/ui-button-drain.png"),
        Vec2::new(15.0, 15.0),
        3,
        1,
        None,
        None,
    ));

    let power_handle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("textures/ui-button-power.png"),
        Vec2::new(15.0, 15.0),
        3,
        1,
        None,
        None,
    ));

    // buttons
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    align_items: AlignItems::End,
                    ..default()
                },
                z_index: ZIndex::Global(i32::MAX - 1),
                ..default()
            },
            HudElement,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect {
                            bottom: Val::Px(PIXEL * 1.0),
                            left: Val::Px(PIXEL * 2.0),
                            ..default()
                        },
                        width: Val::Vw(100.0),
                        height: Val::Px(PIXEL * 15.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    sprite_btn(parent, power_handle, BuildingType::Distribution);
                    sprite_btn(parent, drain_handle, BuildingType::Drain);
                });
        });
}

fn sprite_btn(
    parent: &mut ChildBuilder,
    sprite_atlas_handle: Handle<TextureAtlas>,
    building_type: BuildingType,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(PIXEL * 15.0),
                    height: Val::Px(PIXEL * 15.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..default()
            },
            MenuButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                AtlasImageBundle {
                    texture_atlas: sprite_atlas_handle,
                    style: Style {
                        width: Val::Px(PIXEL * 15.0),
                        height: Val::Px(PIXEL * 15.0),
                        ..default()
                    },
                    ..default()
                },
                building_type,
            ));
        });
}

#[derive(Component, Default)]
struct MenuButton;

fn interact(
    mut placement: ResMut<PlacementState>,
    mut q_btn: Query<(&Interaction, &Children), (Changed<Interaction>, With<MenuButton>)>,

    mut q_sprites: Query<(&mut UiTextureAtlasImage, &BuildingType)>,
) {
    for (interaction, children) in &mut q_btn {
        for child in children {
            if let Ok((mut sprite, building_type)) = q_sprites.get_mut(*child) {
                sprite.index = match *interaction {
                    Interaction::None => {
                        if placement
                            .being_placed_building_type
                            .as_ref()
                            .map_or(false, |val| val == building_type)
                        {
                            2
                        } else {
                            0
                        }
                    }
                    Interaction::Hovered => 1,
                    Interaction::Pressed => {
                        if placement
                            .being_placed_building_type
                            .as_ref()
                            .map_or(false, |val| val == building_type)
                        {
                            placement.being_placed_building_type = None;
                            2
                        } else {
                            placement.being_placed_building_type = Some(building_type.clone());
                            0
                        }
                    }
                };
            }
        }
    }
}

fn set_button_state(
    mut changed: EventReader<PlacementStateChanged>,
    mut q_btn: Query<
        (&mut UiTextureAtlasImage, &BuildingType),
        (With<UiTextureAtlasImage>, With<BuildingType>),
    >,
) {
    for ev in changed.read() {
        if let Some(value) = &ev.value {
            for (mut sprite, b_type) in &mut q_btn {
                if b_type == value && sprite.index != 2 {
                    sprite.index = 2;
                }
            }
        } else {
            for (mut sprite, _) in &mut q_btn {
                if sprite.index != 0 {
                    sprite.index = 0;
                }
            }
        }
    }
}

fn teardown(mut commands: Commands, mut q_elements: Query<Entity, With<HudElement>>) {
    q_elements.for_each(|element| commands.entity(element).despawn_recursive());
}
