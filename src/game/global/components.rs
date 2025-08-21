use crate::prelude::*;

pub struct GlobalPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u64,
}

#[derive(Component)]
pub struct MenuEntity;

#[derive(Component)]
pub struct GameEntity;
