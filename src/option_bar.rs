use crate::{SimulationState, game_of_life::GameOfLife};

pub enum MenuState {
    Main,
    Display,
}

pub fn draw_menu(state: &SimulationState, game: &GameOfLife, menu_state: &MenuState) {
    match menu_state {
        MenuState::Main => {
            print!(
                "[P]ause/Resume | [R]andom | [C]lear | [D]isplay | [Q]uit  --- {}  (step {})\r",
                state.to_string(),
                game.step()
            );
        }
        MenuState::Display => {
            print!(
                "[B]ack | [S]how digits | [1] White | [2] Heatmap | [3] Trail --- {}  (step {})\r",
                state.to_string(),
                game.step()
            );
        }
    }
}
