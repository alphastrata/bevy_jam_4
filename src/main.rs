//! Shows how to render simple primitive shapes with a single color.
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::texture::ImageSamplerDescriptor,
    window::{PresentMode, PrimaryWindow},
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::asset::AssetMetaCheck;

use bevy_hanabi::HanabiPlugin;
use bevy_tweening::TweeningPlugin;

use flora_cause::{
    debug::display_debug::DisplayDebugPlugin,
    game::keybinds::KeybindPlugin,
    global_systems::{
        eargasm::EargasmPlugin, fade_transition::TransitionPlugin, ui_util::UIUtilPlugin,
    },
    scenes::{
        gameplay::GameplayPlugin, menu::MainMenuPlugin, pause::PausePlugin, splash::SplashPlugin,
    },
    AppState, PauseMenuState,
};

fn main() {
    let mut app = App::new();

    if cfg!(target_arch = "wasm32") {
        app.insert_resource(AssetMetaCheck::Never);
    }

    #[cfg(target_arch = "wasm32")]
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            })
            .disable::<ScreenSpaceAmbientOcclusionPlugin>(),
        FrameTimeDiagnosticsPlugin,
        TransitionPlugin,
        TweeningPlugin,
    ));

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }),
        FrameTimeDiagnosticsPlugin,
        TransitionPlugin,
        TweeningPlugin,
    ));

    app.add_plugins((
        KeybindPlugin,
        DisplayDebugPlugin,
        UIUtilPlugin,
        EargasmPlugin,
        HanabiPlugin
    ))
    .add_plugins((SplashPlugin, GameplayPlugin, MainMenuPlugin, PausePlugin))
    .add_state::<AppState>()
    .add_state::<PauseMenuState>()
    .add_systems(Startup, setup);

    app.run();
}

fn setup(_commands: Commands, mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    // unlocks fps with fast vsync (Presentation::Mailbox)
    let mut window = q_window.single_mut();
    window.present_mode = PresentMode::AutoNoVsync;
    #[cfg(not(target_arch = "wasm32"))]
    {
        window.present_mode = PresentMode::AutoNoVsync;
    }

    // window.present_mode = PresentMode::
    info!("{:?}", window.present_mode);
}
