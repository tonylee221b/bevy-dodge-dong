mod dong;
mod player;
mod stats;

use stats::{components::Score, events::ScoreUpEvent};

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>()
        .add_event::<ScoreUpEvent>()
        .add_systems(Startup, player::systems::spawn_player)
        .add_systems(
            Update,
            (player::systems::move_player, player::systems::take_damage),
        )
        .add_systems(
            Update,
            (
                dong::systems::spawn_dong,
                dong::systems::emit_score_up_event,
                dong::systems::despawn_on_collision,
                dong::systems::despawn_on_lifetime_end,
            ),
        )
        .add_systems(Update, stats::systems::score_up_event);
}
