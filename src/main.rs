mod display;
mod game_of_life;
mod option_bar;
mod user_input;

use crate::display::{DisplayConfig, Palette, draw_board};
use crate::game_of_life::GameOfLife;
use crate::option_bar::{MenuState, draw_menu};
use crate::user_input::get_and_execute_user_input;

use termion::async_stdin;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;
use std::time::Instant;

const SIMULATION_SPEED: Duration = Duration::from_millis(70);

enum SimulationState {
    Playing,
    Paused,
    Exiting,
}

impl SimulationState {
    fn to_string(&self) -> &str {
        match self {
            SimulationState::Playing => "Playing",
            SimulationState::Paused => "Paused",
            SimulationState::Exiting => "Exiting",
        }
    }
}

fn force_print() {
    let mut stdout = stdout();
    let _ = stdout.flush();
}

fn main() {
    let mut _stdout = MouseTerminal::from(
        stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap(),
    );
    let stdin = async_stdin();
    let mut events: termion::input::Events<termion::AsyncReader> = stdin.events();

    print!("{}", termion::cursor::Hide);

    let mut game = GameOfLife::new();
    let mut state = SimulationState::Paused;
    let mut menu_state = MenuState::Main;
    let mut config = DisplayConfig {
        show_neighbor_count: false,
        mode: display::Mode::Normal,
        color_palette: Palette::White,
    };
    loop {
        get_and_execute_user_input(
            &mut game,
            &mut state,
            &mut config,
            &mut menu_state,
            &mut events,
        );

        let start = Instant::now();
        draw_board(&game, &config);
        draw_menu(&state, &game, &menu_state);
        force_print();

        match state {
            SimulationState::Paused => {}
            SimulationState::Playing => {
                game.update();
            }
            SimulationState::Exiting => break,
        }

        let duration = start.elapsed();
        if duration >= SIMULATION_SPEED {
            // nothing to do, spent too much time updating
        } else {
            thread::sleep(SIMULATION_SPEED - duration);
        }
    }

    print!("{}", termion::cursor::Show);
}
