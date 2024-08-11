use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
pub struct FPSCounterPlugin;

impl Plugin for FPSCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fps_counter)
            .add_systems(Update, text_update_system);
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

pub fn spawn_fps_counter(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Text with multiple sections
    let a = commands
        .spawn(NodeBundle {
            style: Style {
                right: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(
            |parent| {
                parent.spawn(TextBundle::from_section(
                    "FPS: ",
                    TextStyle {
                        font_size: 60.0,
                        ..default()
                    },
                ));
            }, // Create a TextBundle that has a Text with a list of sections.
        )
        .id();

    commands.entity(a).insert(FpsText);
    //FpsText,
}

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
