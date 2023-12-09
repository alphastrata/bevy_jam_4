use bevy::prelude::*;

use crate::game::camera::main_layer;

#[derive(Resource)]
pub struct GameFont(Handle<Font>);

pub struct UIUtilPlugin;
impl Plugin for UIUtilPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameFont>()
            .add_systems(Update, btn_logic);
    }
}

impl FromWorld for GameFont {
    fn from_world(world: &mut World) -> Self {
        Self(
            world
                .get_resource_mut::<AssetServer>()
                .expect("Why did this not load the asset server?")
                .load("fonts/PixelifySans.ttf"),
        )
    }
}

pub mod btn_styles {
    use bevy::prelude::Color;
    pub(super) const NORMAL: Color = Color::rgb(1.0, 1.0, 01.0);
    pub(super) const HOVERED: Color = Color::rgb(0.85, 0.85, 0.85);
    pub(super) const PRESSED: Color = Color::rgb(0.75, 0.75, 0.75);
}

pub fn img(
    commands: &mut Commands,
    texture: Handle<Image>,
    width: Option<Val>,
    height: Option<Val>,
) -> Entity {
    commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: width.unwrap_or_default(),
                    height: height.unwrap_or_default(),
                    ..default()
                },
                image: UiImage::new(texture),
                ..default()
            },
            main_layer(),
        ))
        .id()
}

pub fn txt(commands: &mut Commands, font: &Res<GameFont>, text: &str, size: f32) -> Entity {
    commands
        .spawn((
            TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: font.0.clone(),
                        font_size: size,
                        color: btn_styles::NORMAL,
                        ..default()
                    },
                ),
                ..default()
            },
            main_layer(),
        ))
        .id()
}

/// Spawn a generic button
pub fn btn(
    commands: &mut Commands,
    font: &Res<GameFont>,
    text: &str,
    action: impl Bundle,
) -> Entity {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(50.0),
                    // border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                // border_color: BorderColor(Color::BLACK),
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..default()
            },
            main_layer(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 32.0,
                        color: btn_styles::NORMAL,
                        ..Default::default()
                    },
                ),
                main_layer(),
            ));
        })
        .insert(action)
        .id()
}

/// Handles changing the button styles
fn btn_logic(
    mut q_int: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut q_text: Query<&mut Text>,
) {
    for (interaction, children) in &mut q_int {
        for child in children {
            if let Ok(mut txt) = q_text.get_mut(*child) {
                for sect in txt.sections.iter_mut() {
                    match *interaction {
                        Interaction::Pressed => {
                            sect.style.color = btn_styles::PRESSED;
                        }
                        Interaction::Hovered => {
                            sect.style.color = btn_styles::HOVERED;
                        }
                        Interaction::None => {
                            sect.style.color = btn_styles::NORMAL;
                        }
                    }
                }
            }
        }
    }
}
