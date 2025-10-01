use termion::event::{Event, Key, MouseEvent};

use crate::SimulationState;
use crate::display::{DisplayConfig, Mode, Palette};
use crate::game_of_life::{GameOfLife, PIXEL_WIDTH};
use crate::option_bar::MenuState;

pub fn get_and_execute_user_input(
    game: &mut GameOfLife,
    state: &mut SimulationState,
    config: &mut DisplayConfig,
    menu_state: &mut MenuState,
    events: &mut termion::input::Events<termion::AsyncReader>,
) {
    for c in events {
        let evt = c.unwrap();
        match evt {
            Event::Key(key) => match key {
                Key::Char('p') | Key::Char('P') => match state {
                    SimulationState::Paused => *state = SimulationState::Playing,
                    SimulationState::Playing => *state = SimulationState::Paused,
                    SimulationState::Exiting => {}
                },
                Key::Char('r') | Key::Char('R') => {
                    game.create_random_board();
                }
                Key::Char('c') | Key::Char('C') => {
                    game.clear_board();
                }
                Key::Char('q') | Key::Char('Q') | Key::Esc | Key::Ctrl('c') => {
                    *state = SimulationState::Exiting;
                    break;
                }
                Key::Char('b') | Key::Char('B') => {
                    *menu_state = MenuState::Main;
                }
                Key::Char('d') | Key::Char('D') => {
                    *menu_state = MenuState::Display;
                }
                Key::Char('s') | Key::Char('S') => {
                    config.show_neighbor_count = !config.show_neighbor_count;
                }
                Key::Char('1') => {
                    config.color_palette = Palette::White;
                    config.mode = Mode::Normal;
                }
                Key::Char('2') => {
                    config.color_palette = Palette::Heatmap;
                    config.mode = Mode::Normal;
                }
                Key::Char('3') => {
                    config.color_palette = Palette::Heatmap;
                    config.mode = Mode::Trail;
                }
                _ => {}
            },
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    if y > game.board().len() as u16
                        || x > game.board()[0].len() as u16 * PIXEL_WIDTH as u16
                    {
                        break;
                    }

                    let true_x = (x - 1) as usize / PIXEL_WIDTH as usize;
                    let true_y = y as usize - 1;

                    game.toggle_cell(true_x, true_y);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
