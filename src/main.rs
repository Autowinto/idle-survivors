use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
struct PlayerData {
    gold: i32,
}

fn add_gold(time: Res<Time>, mut playerData: Query<With<PlayerData>>) {}
