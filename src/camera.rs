use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::DebandDither},
    prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_movement);
    }
}

#[derive(Component)]
pub struct MainCamera;

pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut cam) = camera.get_single_mut() {
        if input.pressed(KeyCode::KeyW) {
            cam.translation.z -= 0.8 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyS) {
            cam.translation.z += 0.8 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyA) {
            cam.translation.x -= 0.8 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyD) {
            cam.translation.x += 0.8 * time.delta_seconds();
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(-8.5, 7.5, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
            deband_dither: DebandDither::Enabled,
            ..Default::default()
        },
        BloomSettings::NATURAL,
        MainCamera,
    ));
}
