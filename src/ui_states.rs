use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameUIStates {
    pub(crate) build_menu_opened: bool,
}
