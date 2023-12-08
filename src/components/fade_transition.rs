use crate::{scenes::pause::PauseState, AppState};
use bevy::{prelude::*, render::view::RenderLayers};

const TRANSITION_DURATION: f32 = 1.0;

struct TransitionParams {
    to: AppState,
}

#[derive(Component)]
struct TransitionComponent;

#[derive(Resource)]
pub struct TransitionState {
    params: Option<TransitionParams>,
}

impl Default for TransitionState {
    fn default() -> Self {
        TransitionState { params: None }
    }
}

pub struct TransitionPlugin;
impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TransitionState>()
            .add_systems(Startup, setup)
            .add_systems(Update, transition);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn((
        NodeBundle {
            z_index: ZIndex::Global(i32::MAX),
            background_color: BackgroundColor(Color::BLACK),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(1),
        TransitionComponent,
    ));
}

fn transition(
    mut state: ResMut<TransitionState>,
    mut q_fade: Query<&mut BackgroundColor, With<TransitionComponent>>,
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    let mut bkg = q_fade.single_mut();

    if let Some(params) = &state.params {
        if bkg.0.a() < 1.0 {
            let next = bkg.0.a() + time.delta_seconds() / TRANSITION_DURATION;
            bkg.0.set_a(next);
        } else if bkg.0.a() > 1.0 {
            bkg.0.set_a(1.0);
        } else if bkg.0.a() == 1.0 {
            app_state.set(params.to);
            state.params = None;
        }
    } else if bkg.0.a() > 0.0 {
        let next = bkg.0.a() - time.delta_seconds() / TRANSITION_DURATION;
        bkg.0.set_a(next);
    } else if bkg.0.a() < 0.0 {
        bkg.0.set_a(0.0);
    }
}

pub fn transition_to(app_state: AppState, transition_state: &mut ResMut<TransitionState>) {
    transition_state.params = Some(TransitionParams { to: app_state });
}
