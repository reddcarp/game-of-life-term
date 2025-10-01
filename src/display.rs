use termion::color;
use termion::color::{Bg, Fg, Rgb};

use crate::game_of_life::{GameOfLife, PIXEL_WIDTH, TRAIL_LIFETIME_MAX};

#[derive(Clone)]
pub enum Palette {
    Heatmap,
    White,
}

#[derive(Clone)]
pub enum Mode {
    Normal,
    Trail,
}

pub struct DisplayConfig {
    pub show_neighbor_count: bool,
    pub mode: Mode,
    pub color_palette: Palette,
}

fn bg_color_neighbors_map(n: i8, palette: &Palette) -> String {
    match palette {
        Palette::White => format!("{}", Bg(color::Reset)),
        Palette::Heatmap => {
            let color_map = [
                Rgb(0, 0, 0),
                Rgb(0, 0, 255),
                Rgb(0, 128, 255),
                Rgb(0, 255, 255),
                Rgb(0, 255, 128),
                Rgb(0, 255, 0),
                Rgb(255, 255, 0),
                Rgb(255, 128, 0),
                Rgb(255, 0, 0),
            ];
            let color = color_map[n.min(8) as usize];

            if color == Rgb(0, 0, 0) {
                format!("{}", Bg(color::Reset))
            } else {
                format!("{}", Bg(color))
            }
        }
    }
}

fn fg_color_neighbors_map(n: i8, palette: &Palette) -> Fg<Rgb> {
    let color_map = match palette {
        Palette::White => [
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
        ],
        Palette::Heatmap => [
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
            Rgb(0, 0, 0),
            Rgb(0, 0, 0),
            Rgb(0, 0, 0),
            Rgb(0, 0, 0),
            Rgb(255, 255, 255),
            Rgb(255, 255, 255),
        ],
    };

    let color = color_map[n.min(8) as usize];
    Fg(color)
}

fn bg_color_trail_map(lifetime: u8) -> String {
    if lifetime == 0 {
        return format!("{}", Bg(color::Reset));
    }

    let t = (lifetime as f32) / (TRAIL_LIFETIME_MAX as f32);

    // Rainbow gradient: red -> yellow -> green -> cyan -> blue
    let r = ((1.0 - t) * 255.0) as u8;
    let g = (t * 255.0) as u8;
    let b = ((0.5 - (t - 0.5).abs()) * 255.0 * 2.0) as u8;

    format!("{}", Bg(Rgb(r, g, b)))
}

fn fg_color_trail_map(_: u8) -> String {
    format!("{}", Fg(Rgb(255, 255, 255)))
}

// TERMINAL
//   1
// 1 +-------------->x
//   |
//   |
//   |
//   |
//   y
pub fn draw_board(game: &GameOfLife, config: &DisplayConfig) {
    let pixel_left = std::iter::repeat(' ')
        .take(PIXEL_WIDTH as usize / 2)
        .collect::<String>();
    let pixel_right = std::iter::repeat(' ')
        .take(PIXEL_WIDTH as usize - (PIXEL_WIDTH as usize / 2) - 1)
        .collect::<String>();
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    for row in game.board() {
        for cell in row {
            match cell.alive {
                true => print!("{}{}", Bg(color::White), Fg(color::Black)),
                false => match config.mode {
                    Mode::Normal => print!(
                        "{}{}",
                        bg_color_neighbors_map(cell.neighbors, &config.color_palette),
                        fg_color_neighbors_map(cell.neighbors, &config.color_palette)
                    ),
                    Mode::Trail => print!(
                        "{}{}",
                        bg_color_trail_map(cell.trail_lifetime),
                        fg_color_trail_map(cell.trail_lifetime)
                    ),
                },
            }

            if config.show_neighbor_count {
                print!(
                    "{pixel_left}{}{pixel_right}",
                    if cell.neighbors == 0 {
                        " ".to_string()
                    } else {
                        cell.neighbors.to_string()
                    }
                );
            } else {
                print!("{pixel_left} {pixel_right}");
            }
        }
        println!("{}{}\r", Bg(color::Reset), Fg(color::Reset));
    }
}
