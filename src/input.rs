use bevy::app::{App, Plugin};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        let _ = app;
        //app.add_systems(Update, keyboard_input);
    }
}
