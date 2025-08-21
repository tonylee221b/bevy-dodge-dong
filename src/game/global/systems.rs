use crate::{
    game::global::components::{GameState, MenuEntity},
    prelude::*,
};

use super::components::{GameEntity, GlobalPlugin, Score};

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game)
            .add_systems(OnExit(GameState::InGame), cleanup_game)
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

fn setup_game(mut commands: Commands) {
    commands.init_resource::<Score>();
}

fn cleanup_game(mut commands: Commands, game_entities: Query<Entity, With<GameEntity>>) {
    for entity in game_entities.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<Score>();
}

fn cleanup_menu(mut commands: Commands, menu_entities: Query<Entity, With<MenuEntity>>) {
    for entity in menu_entities.iter() {
        commands.entity(entity).despawn();
    }
}
