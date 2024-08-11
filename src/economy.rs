use bevy::{self, prelude::Resource};

#[derive(Resource)]
struct Economy {
    money: f32,
    gold: f32,
}
