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
    mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
    utils::default,
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
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            bevy::asset::RenderAssetUsages::default(),
        );
        for ((x, y, z), &block) in chunk.blocks.indexed_iter() {
            if block == BlockType::Air {
                continue;
            }
            let x = x as f32 + chunk.pos_x as f32;
            let y = y as f32;
            let z = z as f32 + chunk.pos_z as f32;

            let start = vertices.len() as u32;

            vertices.extend_from_slice(&[
                [x, y, z],                // 0 bottom back left
                [x + 1., y, z],           // 1 bottom back right
                [x, y, z + 1.],           // 2 bottom front left
                [x + 1., y, z + 1.],      // 3 bottom front right
                [x, y + 1., z],           // 4 top back left
                [x + 1., y + 1., z],      // 5 top back right
                [x, y + 1., z + 1.],      // 6 top front left
                [x + 1., y + 1., z + 1.], // 7 top front right
            ]);
            let faces = [
                [0, 1, 3, 2], // Bottom
                [4, 6, 7, 5], // Top
                [0, 2, 6, 4], // Left
                [1, 5, 7, 3], // Right
                [2, 3, 7, 6], // Front
                [0, 4, 5, 1], // Back
            ];

            for face in faces {
                indices.extend_from_slice(&[
                    start + face[0],
                    start + face[1],
                    start + face[2],
                    start + face[0],
                    start + face[2],
                    start + face[3],
                ]);
            }
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
        let material = match chunk.blocks.iter().find(|&&block| block != BlockType::Air) {
            Some(BlockType::Grass) => StandardMaterial { ..default() },
            _ => StandardMaterial { ..default() },
        };
        commands.entity(entity).insert((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(material)),
            ChunkRendered,
        ));
    }
}
