use bevy::prelude::*;

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(30.0, 30.0),
        )),
        material: materials.add(Color::srgb_u8(0, 120, 0)),
        //transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 4.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(4.0, 2.0, 5.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 3.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(3.0, 1.5, 2.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb_u8(255, 228, 156),
            shadows_enabled: true,
            range: 1000.0,
            intensity: 10000000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        ..default()
    });
    // camera
    // commands.spawn(camera::Ca {
    //     transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}

pub fn spawn_test_buildings(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let mesh_water: Handle<_> = _asset_server.load("models/water.glb#Scene0");
    let mesh_energy: Handle<_> = _asset_server.load("models/energy.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: mesh_water,
        transform: Transform::from_xyz(2.0, 2.0, 1.0),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: mesh_energy,
        transform: Transform::from_xyz(4.0, 2.0, 3.0),
        ..Default::default()
    });
}
