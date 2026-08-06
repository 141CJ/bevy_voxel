use bevy::{
    app::{App, Plugin, Startup},
    camera::Camera3d,
    camera_controller::free_camera::FreeCamera,
    ecs::system::Commands,
    math::Vec3,
    transform::components::Transform,
    utils::default,
};

pub struct FreeCam;
impl Plugin for FreeCam {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 90., 0.).looking_to(Vec3::X, Vec3::Y),
        FreeCamera {
            sensitivity: 0.2,
            friction: 25.,
            walk_speed: 10.,
            run_speed: 20.,
            ..default()
        },
    ));
}
