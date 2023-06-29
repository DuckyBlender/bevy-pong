use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

// import consts from main.rs
use crate::{EguiContexts, GameState, egui};

pub enum ButtonType {
    Singleplayer,
    Multiplayer,
    Quit,
}

#[derive(Component)]
pub struct Button {
    button_type: ButtonType,
}

pub fn close_menu(buttons: Query<Entity, With<Button>>, mut commands: Commands) {
    // Remove the UI Buttons (with the Button component)
    for button in buttons.iter() {
        commands.entity(button).despawn_recursive();
    }
}

pub fn menu_system(
    mut contexts: EguiContexts,
    _commands: Commands,
    mut state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Main Menu");

        ui.vertical_centered_justified(|ui| {
            if ui.button("Singleplayer").clicked() {
                state.set(GameState::Singleplayer);
            }

            if ui.button("Multiplayer").clicked() {
                state.set(GameState::Multiplayer);
            }

            if ui.button("Quit").clicked() {
                // Force quit
                std::process::exit(0);
            }
        });
    });
}
