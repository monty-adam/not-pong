extern crate termion;

use std::{
    cmp,
    collections::HashSet,
    io::{self, Write},
    time::{Duration, Instant},
};
use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};

fn draw_paddle<W: Write>(screen: &mut W, paddle: &Paddle) {
    write!(
        screen,
        "{}\u{2588}{}\u{2588}{}\u{2588}",
        termion::cursor::Goto(paddle.x_coordinate, paddle.y_coordinate),
        termion::cursor::Goto(paddle.x_coordinate, paddle.y_coordinate + 1),
        termion::cursor::Goto(paddle.x_coordinate, paddle.y_coordinate + 2),
    )
    .unwrap();
}

fn draw_ball<W: Write>(screen: &mut W, ball: &Ball) {
    write!(
        screen,
        "{}\u{2588}{}\u{2588}",
        termion::cursor::Goto(ball.x_coordinate, ball.y_coordinate + 1),
        termion::cursor::Goto(ball.x_coordinate + 1, ball.y_coordinate + 1),
    )
    .unwrap();
}

enum BallDirection {
    ToPlayer,
    FromPlayer,
}
struct Ball {
    x_coordinate: u16,
    y_coordinate: u16,
    direction: BallDirection,
}

impl Ball {
    fn new(x_coordinate: u16, y_coordinate: u16) -> Self {
        Self {
            x_coordinate,
            y_coordinate,
            direction: BallDirection::FromPlayer,
        }
    }

    fn check_collision(&mut self, paddle: &Paddle) {
        let is_touching = self
            .get_hitbox()
            .is_disjoint(&paddle.get_hitbox());

        if is_touching {
            if self.x_coordinate == paddle.x_coordinate + 1 {
                self.direction = BallDirection::FromPlayer;
            } else if self.x_coordinate == paddle.x_coordinate - 2 {
                self.direction = BallDirection::ToPlayer;
            };
        };
    }

    fn get_hitbox(&self) -> HashSet<u16> {
        HashSet::from([self.x_coordinate, self.x_coordinate + 1])
    }
}

struct Paddle {
    x_coordinate: u16,
    y_coordinate: u16,
}

impl Paddle {
    fn new_player(y_coordinate: u16) -> Self {
        Self {
            x_coordinate: 4,
            y_coordinate,
        }
    }

    fn new_rival(x_coordinate: u16, y_coordinate: u16) -> Self {
        Self {
            x_coordinate,
            y_coordinate,
        } 
    }

    fn get_hitbox(&self) -> HashSet<u16> {
        HashSet::from([self.y_coordinate, self.y_coordinate + 1, self.y_coordinate + 2])
    }
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

        write!(screen, "{}{}", termion::clear::All, termion::cursor::Hide,).unwrap();

        // Starting variables
        // let mut player_location = 1;
        // let mut ball_location = 5;
        // let mut move_right = true;
        let mut player = Paddle::new_player(1);
        let rival = Paddle::new_rival(term_width, 1);
        let mut ball = Ball::new(5, 1);

        // Init game
        let mut start = Instant::now();
        draw_paddle(&mut screen, &player);
        draw_paddle(&mut screen, &rival);
        draw_ball(&mut screen, &ball);
        screen.flush().unwrap();

        loop {
            let duration = start.elapsed();
            if duration >= Duration::from_millis(33) {
                start = Instant::now();

                ball.x_coordinate = match ball.direction {
                    BallDirection::FromPlayer => cmp::min(ball.x_coordinate + 1, term_width),
                    BallDirection::ToPlayer => cmp::max(ball.x_coordinate - 1, term_width / term_width),
                };

                write!(screen, "{}", termion::clear::All).unwrap();
                draw_paddle(&mut screen, &player);
                draw_paddle(&mut screen, &rival);
                draw_ball(&mut screen, &ball);
                screen.flush().unwrap();
            }

            ball.check_collision(&player);
            ball.check_collision(&rival);

            if let Some(c) = keys_pressed.next() {
                player.y_coordinate = match c.unwrap() {
                    Key::Char('q') => break,
                    Key::Char('z') => cmp::max(player.y_coordinate - 1, 1),
                    Key::Char('x') => cmp::min(player.y_coordinate + 1, term_height - 3),
                    Key::Null | _ => player.y_coordinate,
                };
            };
        }
    }
}
