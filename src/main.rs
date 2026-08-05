use bevy::{
    camera_controller::free_camera::FreeCameraPlugin,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    state::commands,
};
use ndarray::Array3;

mod freecam;

#[derive(Clone, Copy, PartialEq)]
enum BlockType {
    Air,
    Grass,
    Stone,
}

struct Chunk {
    length: usize,
    height: usize,
    blocks: Array3<BlockType>,
}

impl Chunk {
    fn new(length: usize, height: usize) -> Self {
        Chunk {
            blocks: Array3::from_elem((length, height, length), BlockType::Air),
            length,
            height,
        }
    }
    // Converts XYZ coordinates to flat vector coordinates
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + (y * self.length) + (z * self.height * self.height)
    }

    fn get(&self, x: usize, y: usize, z: usize) -> Option<&BlockType> {
        self.blocks.get((x, y, z))
    }

    fn set(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if let Some(cell) = self.blocks.get_mut((x, y, z)) {
            *cell = block;
        }
    }
}

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
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, scene)
        .add_systems(Update, overlay_config)
        .add_plugins(freecam::FreeCam)
        .run();
}

fn scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunk = Chunk::new(16, 64);
    draw_chunk(commands, meshes, materials, &chunk);
}

fn draw_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk: &Chunk,
) {
    for ((x, y, z), &block) in chunk.blocks.indexed_iter() {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
            Transform::from_xyz(x as f32, y as f32, z as f32),
        ));
    }
}

fn overlay_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
        overlay.frame_time_graph_config.enabled = !overlay.frame_time_graph_config.enabled;
    }
}
