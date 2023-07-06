extern crate termion;

use std::{io::{self, Write}, cmp};
use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::{IntoAlternateScreen, ToAlternateScreen, ToMainScreen},
};

fn draw_player_paddle<W: Write>(screen: &mut W, y_coordinate: u16) {
    write!(
        screen,
        "{}{}\u{2588}{}\u{2588}{}\u{2588}",
        termion::clear::All,
        termion::cursor::Goto(4, y_coordinate),
        termion::cursor::Goto(4, y_coordinate + 1),
        termion::cursor::Goto(4, y_coordinate + 2),
    ).unwrap();
}

fn draw_ball<W: Write>(screen: &mut W, x_coordinate: u16, y_coordinate: u16) {
    write!(
        screen,
        "{}\u{2588}{}\u{2588}",
        termion::cursor::Goto(x_coordinate, y_coordinate + 1),
        termion::cursor::Goto(x_coordinate + 1, y_coordinate + 1),
    ).unwrap();
}

fn main() {
    let stdout = io::stdout().lock();
    let stdin = io::stdin().lock();
    let (term_height, term_width) = termion::terminal_size().unwrap();
    let term_height = cmp::min(term_height, 79);

    {
        let mut screen = stdout
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        screen.flush().unwrap();
        write!(
            screen,
            "{}{}",
            termion::clear::All,
            termion::cursor::Hide,
        ).unwrap();

        let mut player_location = 1;
        draw_player_paddle(&mut screen, player_location);

        let mut ball_location = 5;

        for c in stdin.keys() {
            screen.flush().unwrap();
            draw_player_paddle(&mut screen, player_location);

            // Rival paddle placeholder
            write!(
                screen,
                "{}\u{2588}{}\u{2588}{}\u{2588}",
                termion::cursor::Goto(term_width, 1),
                termion::cursor::Goto(term_width, 2),
                termion::cursor::Goto(term_width, 3),
            ).unwrap();

            draw_ball(&mut screen, ball_location, player_location);

            player_location = match c.unwrap() {
                Key::Char('q') => break,
                Key::Char('z') => cmp::max(player_location - 1, 1),
                Key::Char('x') => cmp::min(player_location + 1, term_height - 3),
                _ => player_location,
            };

            ball_location = cmp::min(ball_location + 1, term_width - 2);
        }
    }
}
