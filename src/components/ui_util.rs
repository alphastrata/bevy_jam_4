use bevy::prelude::*;

pub struct UIUtilPlugin;
impl Plugin for UIUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, btn_logic);
    }
}

pub mod button_styles {
    use bevy::prelude::Color;
    pub(super) const NORMAL: Color = Color::rgb(1.0, 1.0, 01.0);
    pub(super) const HOVERED: Color = Color::rgb(0.85, 0.85, 0.85);
    pub(super) const PRESSED: Color = Color::rgb(0.75, 0.75, 0.75);
}

pub fn txt(commands: &mut Commands, text: &str, size: f32) -> Entity {
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size: size,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ),
            ..default()
        })
        .id()
}

/// Spawn a generic button
pub fn btn(commands: &mut Commands, text: &str, action: impl Bundle) -> Entity {
    commands
        .spawn(ButtonBundle {
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
            // background_color: button_styles::NORMAL.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    // TODO: load font - font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ));
        })
        .insert(action)
        .id()
}

/// Handles changing the button styles
fn btn_logic(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    interaction_query
        .iter_mut()
        .for_each(|(_interaction, children)| {
            children.iter().for_each(|child| {
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.sections.iter_mut().for_each(|section| {
                        section.style.color = button_styles::PRESSED.into();
                    });
                }

                // match *interaction {
                //     Interaction::Pressed => {
                //         *color = button_styles::PRESSED.into();
                //         border_color.0 = Color::RED;
                //     }
                //     Interaction::Hovered => {
                //         *color = button_styles::HOVERED.into();
                //         border_color.0 = Color::WHITE;
                //     }
                //     Interaction::None => {
                //         *color = button_styles::NORMAL.into();
                //         border_color.0 = Color::BLACK;
                //     }
                // }
            });
        });
}
