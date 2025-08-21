mod dong;
mod global;
mod player;
mod ui;

use dong::components::DongPlugin;
use global::components::{GameState, GlobalPlugin};
use player::components::PlayerPlugin;
use ui::components::UIPlugin;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_plugins(GlobalPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(DongPlugin);
}
