use bevy::{
    camera_controller::free_camera::FreeCameraPlugin,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    state::commands,
};
use ndarray::Array3;
use noise::{NoiseFn, Perlin};

mod freecam;

#[derive(Clone, Copy, PartialEq, Debug)]
enum BlockType {
    Air,
    Grass,
    Stone,
}

struct Chunk {
    length: usize,
    height: usize,
    blocks: Array3<BlockType>,
    scale: f64, // Noise scale
    amp: f64,   // Noise amplitude
    noise: Perlin,
}

impl Chunk {
    fn new(length: usize, height: usize, seed: u32) -> Self {
        Chunk {
            blocks: Array3::from_elem((length, height, length), BlockType::Air),
            scale: 32.,
            amp: 32.,
            noise: Perlin::new(seed),
            length,
            height,
        }
    }

    fn generate(&mut self) {
        for x in 0..self.length {
            for z in 0..self.length {
                let tall = self
                    .noise
                    .get([x as f64 / self.scale, z as f64 / self.scale]);
                let terrain_height = (tall * self.amp as f64 + (self.height / 2) as f64) as i32;

                for y in 0..self.height {
                    if y < terrain_height as usize {
                        self.set(x, y, z, BlockType::Stone);
                    } else if y == terrain_height as usize {
                        self.set(x, y, z, BlockType::Grass);
                    } else {
                    }
                }
            }
        }
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
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 500.,
            ..default()
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
    let mut chunk = Chunk::new(16, 64, 42);
    chunk.generate();
    draw_chunk(commands, meshes, materials, &chunk);
}

fn draw_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk: &Chunk,
) {
    for ((x, y, z), &block) in chunk.blocks.indexed_iter() {
        if block == BlockType::Grass {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
                Transform::from_xyz(x as f32, y as f32, z as f32),
            ));
        }
        if block == BlockType::Stone {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(materials.add(Color::srgb_u8(115, 110, 115))),
                Transform::from_xyz(x as f32, y as f32, z as f32),
            ));
        }
        println!("{:?}", block);
    }
}

fn overlay_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
        overlay.frame_time_graph_config.enabled = !overlay.frame_time_graph_config.enabled;
    }
}
