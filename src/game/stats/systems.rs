use crate::prelude::*;

use super::{components::Score, events::ScoreUpEvent};

pub fn score_up_event(mut score: ResMut<Score>, mut score_events: EventReader<ScoreUpEvent>) {
    for _ in score_events.read() {
        score.value += 1;
        println!("Score: {:#?}", score.value);
    }
}
