use crate::{
    game::{
        global::components::{GameState, Score},
        player::components::PlayerHealth,
    },
    prelude::*,
};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use super::components::UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            (
                main_menu_ui.run_if(in_state(GameState::MainMenu)),
                game_ui.run_if(in_state(GameState::InGame)),
                game_over_ui.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}

fn main_menu_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) -> Result {
    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);

            ui.heading(
                egui::RichText::new("Dodge Poop~~!!!")
                    .size(48.0)
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(50.0);

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Play"))
                .clicked()
            {
                next_state.set(GameState::InGame);
            }

            ui.add_space(20.0);
            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Quit"))
                .clicked()
            {
                exit.write(AppExit::Success);
            }
        });
    });
    Ok(())
}

fn game_ui(
    mut contexts: EguiContexts,
    score: Option<Res<Score>>,
    health: Option<Res<PlayerHealth>>,
) -> Result {
    if let (Some(score), Some(health)) = (score, health) {
        egui::TopBottomPanel::top("score_panel").show(contexts.ctx_mut()?, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() - 300.0);

                ui.vertical(|ui| {
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Health: ")
                                .size(20.0)
                                .color(egui::Color32::WHITE),
                        );

                        for i in 0..3 {
                            let heart_color = if i < health.cnt {
                                egui::Color32::RED
                            } else {
                                egui::Color32::DARK_GRAY
                            };

                            ui.label(egui::RichText::new("❤").size(24.0).color(heart_color));
                        }
                    });

                    ui.add_space(5.0);

                    ui.label(
                        egui::RichText::new(format!("Score: {}", score.value))
                            .size(24.0)
                            .color(egui::Color32::WHITE),
                    );
                });
            });
        });
    }
    Ok(())
}

fn game_over_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    score: Option<Res<Score>>,
) -> Result {
    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);

            ui.label(
                egui::RichText::new("🎮 GAME OVER 🎮")
                    .size(48.0)
                    .color(egui::Color32::RED),
            );

            ui.add_space(30.0);

            if let Some(score) = score {
                ui.label(
                    egui::RichText::new(format!("Final Score: {}", score.value))
                        .size(32.0)
                        .color(egui::Color32::WHITE),
                );
            }

            ui.add_space(50.0);

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Retry"))
                .clicked()
            {
                next_state.set(GameState::InGame);
            }

            ui.add_space(20.0);

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Main Menu"))
                .clicked()
            {
                next_state.set(GameState::MainMenu);
            }
        });
    });

    Ok(())
}
