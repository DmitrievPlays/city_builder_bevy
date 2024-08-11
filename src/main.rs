mod bundles;
mod camera;
mod economy;
mod fps_counter;
mod input;
mod scene;
mod ui;
mod ui_states;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, winit::WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use input::InputPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "City Builder Game".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..Default::default()
                }),
            FrameTimeDiagnosticsPlugin,
            CameraPlugin,
            UIPlugin,
            //FPSCounterPlugin,
            InputPlugin,
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, add_buildings)
        .add_systems(Startup, (scene::setup, scene::spawn_test_buildings))
        .add_systems(Update, get_buildings)
        .run();
}

#[derive(Component)]
struct Structure {
    _name: String,
    _description: String,
}

#[derive(Component)]
#[allow(dead_code)]
struct EnergyProvider(f32);

fn add_buildings(mut commands: Commands) {
    commands.spawn((
        Structure {
            _name: "B1".to_string(),
            _description: "B1_desc".to_string(),
        },
        EnergyProvider(2.0),
    ));
    commands.spawn((
        Structure {
            _name: "B2".to_string(),
            _description: "B2_desc".to_string(),
        },
        EnergyProvider(2.0),
    ));
}

fn get_buildings(query: Query<&Structure, With<EnergyProvider>>) {
    for _structure in &query {
        //println!("hello {}!", structure.name);
    }
}
