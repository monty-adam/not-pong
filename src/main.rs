extern crate termion;

use std::{io::{self, Write}, time::{Duration, Instant}, cmp};
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

fn draw_rival_paddle<W: Write>(screen: &mut W, x_coordinate: u16, y_coordinate: u16) {
    write!(
        screen,
        "{}\u{2588}{}\u{2588}{}\u{2588}",
        termion::cursor::Goto(x_coordinate, y_coordinate),
        termion::cursor::Goto(x_coordinate, y_coordinate + 1),
        termion::cursor::Goto(x_coordinate, y_coordinate + 2),
    ).unwrap();
}

fn main() {
    let stdout = io::stdout().lock();
    let stdin = termion::async_stdin();
    let (term_height, term_width) = termion::terminal_size().unwrap();
    let term_height = cmp::min(term_height, 79);

    {
        let mut screen = stdout
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        let mut keys_pressed = stdin.keys();
        
        write!(
            screen,
            "{}{}",
            termion::clear::All,
            termion::cursor::Hide,
        ).unwrap();

        // Starting variables
        let mut player_location = 1;
        let mut ball_location = 5;
        let mut move_right = true;

        // Init game
        let mut start = Instant::now();
        screen.flush().unwrap();
        draw_player_paddle(&mut screen, player_location);
        draw_rival_paddle(&mut screen, term_width, 1);
        draw_ball(&mut screen, ball_location, player_location);

        loop {
            let duration = start.elapsed();
            if duration >= Duration::from_millis(33) {
                start = Instant::now();

                if ball_location == 5 {
                    move_right = true;
                } else if ball_location == term_width - 2 {
                    move_right = false;
                };
    
                ball_location = if move_right {
                    cmp::min(ball_location + 1, term_width - 2)
                } else {
                    cmp::max(ball_location - 1, 5)
                };

                screen.flush().unwrap();
                draw_player_paddle(&mut screen, player_location);
                draw_rival_paddle(&mut screen, term_width, 1);
                draw_ball(&mut screen, ball_location, player_location);
            }

            if let Some(c) = keys_pressed.next() {
                player_location = match c.unwrap() {
                    Key::Char('q') => break,
                    Key::Char('z') => cmp::max(player_location - 1, 1),
                    Key::Char('x') => cmp::min(player_location + 1, term_height - 3),
                    Key::Null | _ => player_location,
                };
            };
        }
    }
}
