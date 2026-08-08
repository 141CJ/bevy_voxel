use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::Without,
        resource::Resource,
        system::{Commands, Query, ResMut},
    },
    math::primitives::Cuboid,
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
};
use ndarray::Array3;
use noise::{NoiseFn, Perlin};

#[derive(Clone, Copy, PartialEq, Debug)]
enum BlockType {
    Air,
    Grass,
    Stone,
}

#[derive(Component)]
pub struct Chunk {
    length: usize,
    height: usize,
    pos_x: usize,
    pos_z: usize,
    blocks: Array3<BlockType>,
    scale: f64, // Noise scale
    amp: f64,   // Noise amplitude
    noise: Perlin,
}

#[derive(Resource)]
pub struct ChunkGenerated;

#[derive(Component)]
pub struct ChunkRendered;

impl Chunk {
    pub fn new(length: usize, height: usize, x: usize, z: usize, seed: u32) -> Self {
        Chunk {
            blocks: Array3::from_elem((length, height, length), BlockType::Air),
            scale: 100.,
            amp: 32.,
            noise: Perlin::new(seed),
            length,
            height,
            pos_x: x * length,
            pos_z: z * length,
        }
    }

    fn generate(&mut self) {
        for x in 0..self.length {
            for z in 0..self.length {
                let chunk_x = x + self.pos_x;
                let chunk_z = z + self.pos_z;

                let tall = self
                    .noise
                    .get([chunk_x as f64 / self.scale, chunk_z as f64 / self.scale]);
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

pub fn generate(mut query: Query<&mut Chunk, Without<ChunkRendered>>) {
    for mut chunk in query.iter_mut() {
        chunk.generate();
    }
}

pub fn render_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Chunk), Without<ChunkRendered>>,
) {
    for (entity, chunk) in query {
        for ((x, y, z), &block) in chunk.blocks.indexed_iter() {
            if block == BlockType::Grass {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::default())),
                    MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
                    Transform::from_xyz(
                        x as f32 + chunk.pos_x as f32,
                        y as f32,
                        z as f32 + chunk.pos_z as f32,
                    ),
                ));
            }
            if block == BlockType::Stone {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::default())),
                    MeshMaterial3d(materials.add(Color::srgb_u8(115, 110, 115))),
                    Transform::from_xyz(
                        x as f32 + chunk.pos_x as f32,
                        y as f32,
                        z as f32 + chunk.pos_z as f32,
                    ),
                ));
            }
        }
        commands.entity(entity).insert(ChunkRendered);
    }
}
