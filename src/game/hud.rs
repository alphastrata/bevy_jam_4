use bevy::prelude::*;

use crate::AppState;

#[derive(Component, Default)]
struct HudElement;

const PIXEL: f32 = 4.0;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), setup)
            .add_systems(OnExit(AppState::Gameplay), teardown);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                            bottom: Val::Px(PIXEL),
                            left: Val::Px(PIXEL * 2.0),
                            ..default()
                        },
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
}

fn teardown(mut commands: Commands, mut q_elements: Query<Entity, With<HudElement>>) {
    q_elements.for_each(|element| commands.entity(element).despawn_recursive());
}
