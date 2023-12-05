use bevy::prelude::*;

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, btn_logic);
    }
}

pub mod button_styles {
    use bevy::prelude::Color;
    pub(super) const NORMAL: Color = Color::rgb(0.15, 0.15, 0.15);
    pub(super) const HOVERED: Color = Color::rgb(0.30, 0.50, 0.30);
    pub(super) const PRESSED: Color = Color::rgb(0.35, 0.35, 0.35);
}

/// Spawn a generic button
pub fn spawn_button(commands: &mut Commands, text: &str, action: impl Bundle) -> Entity {
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
        .insert(action)
        .id()
}

/// Handles changing the button styles
fn btn_logic(
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
