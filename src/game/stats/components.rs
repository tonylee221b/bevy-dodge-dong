use crate::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
pub struct ScoreLine {
    pub pos: Vec2,
}
