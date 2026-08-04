use bevy::{
    camera_controller::free_camera::FreeCameraPlugin,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

mod freecam;

#[derive(Clone, Copy)]
enum BlockType {
    Air,
    Grass,
    Stone,
}

struct Chunk {
    length: usize,
    height: usize,
    blocks: Vec<BlockType>,
}

impl Chunk {
    fn new(length: usize, height: usize) -> Self {
        Chunk {
            blocks: vec![BlockType::Air; length * height * length],
            length,
            height,
        }
    }
    // Converts XYZ coordinates to flat vector coordinates
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + (y * self.length) + (z * self.height * self.height)
    }

    fn get(&self, x: usize, y: usize, z: usize) -> BlockType {
        self.blocks[self.index(x, y, z)]
    }

    fn set(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        let pos = self.index(x, y, z);
        self.blocks[pos] = block;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, overlay_config)
        .add_plugins(freecam::FreeCam)
        .run();
}

fn scene() -> impl SceneList {
    bsn_list! [
        (
            #CircularBase
            Mesh3d(asset_value(Circle::new(4.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
        ),
        (
            #Cube
            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(0, 255, 0)))
            Transform::from_xyz(0.0, 0.5, 0.0)
        ),
        (
            PointLight {
                shadow_maps_enabled: true,
            }
            Transform::from_xyz(4.0, 8.0, 4.0)
        ),
        (
            // Camera3d
            // template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
        )
    ]
}

fn overlay_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
        overlay.frame_time_graph_config.enabled = !overlay.frame_time_graph_config.enabled;
    }
}
