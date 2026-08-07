use bevy::{
    camera_controller::free_camera::FreeCameraPlugin,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

use crate::chunk::{Chunk, ChunkGenerated};

mod chunk;
mod freecam;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: FontSize::Px(42.0),
                    ..default()
                },
                frame_time_graph_config: bevy::dev_tools::fps_overlay::FrameTimeGraphConfig {
                    enabled: true,
                    min_fps: 100.,
                    target_fps: 144.,
                },
                ..default()
            },
        })
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 500.,
            ..default()
        })
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, scene)
        .add_systems(
            Update,
            (
                overlay_config,
                chunk::generate.run_if(resource_exists::<ChunkGenerated>),
                chunk::render_chunk.run_if(resource_exists::<ChunkGenerated>),
            ),
        )
        .add_plugins(freecam::FreeCam)
        .run();
}

fn scene(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,

            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
    ));
    commands.spawn(Chunk::new(16, 64, 42));
    commands.spawn(ChunkGenerated);
}

fn overlay_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
        overlay.frame_time_graph_config.enabled = !overlay.frame_time_graph_config.enabled;
    }
}
