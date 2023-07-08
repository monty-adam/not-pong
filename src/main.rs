extern crate rand;
extern crate termion;

use rand::{thread_rng, Rng};
use std::{
    cmp,
    collections::HashSet,
    io::{self, Write},
    time::{Duration, Instant},
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen};

const DIFICULTY: u16 = 12;  // Easy 10; Medium 8; Hard 12

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

enum BallBounce {
    Up,
    Down,
    Forward,
}
struct Ball {
    x_coordinate: u16,
    y_coordinate: u16,
    direction: BallDirection,
    bounce: BallBounce,
}

impl Ball {
    fn new(x_coordinate: u16, y_coordinate: u16) -> Self {
        Self {
            x_coordinate,
            y_coordinate,
            direction: BallDirection::FromPlayer,
            bounce: BallBounce::Forward,
        }
    }

    fn _bounce(&self) -> BallBounce {
        match thread_rng().gen_range(0..=3) {
            0 => BallBounce::Up,
            1 => BallBounce::Down,
            _ => BallBounce::Forward,
        }
    }

    fn check_collision(&mut self, paddle: &Paddle) {
        if paddle.is_touching(self) {
            if self.x_coordinate == paddle.x_coordinate + 1 {
                self.direction = BallDirection::FromPlayer;
                self.bounce = self._bounce();
            } else if self.x_coordinate == paddle.x_coordinate - 2 {
                self.direction = BallDirection::ToPlayer;
                self.bounce = self._bounce();
            };
        };
    }

    fn _graduate_bounce(&self, y_coordinate: u16) -> u16 {
        if self.x_coordinate % 8 == 0 {
            y_coordinate
        } else {
            self.y_coordinate
        }
    }

    fn move_towards(&mut self, max_x: u16, max_y: u16) {
        self.y_coordinate = match self.bounce {
            BallBounce::Up => self._graduate_bounce(cmp::max(self.y_coordinate - 1, 1)),
            BallBounce::Down => self._graduate_bounce(cmp::min(self.y_coordinate + 1, max_y)),
            BallBounce::Forward => self.y_coordinate,
        };

        self.x_coordinate = match self.direction {
            BallDirection::FromPlayer => cmp::min(self.x_coordinate + 1, max_x),
            BallDirection::ToPlayer => cmp::max(self.x_coordinate - 1, 1),
        };
    }

    fn get_hitbox(&self) -> HashSet<u16> {
        HashSet::from([self.y_coordinate + 1, self.y_coordinate + 1])
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
        HashSet::from([
            self.y_coordinate,
            self.y_coordinate + 1,
            self.y_coordinate + 2,
        ])
    }

    fn catch_ball(&mut self, y_coordinate: u16) {
        self.y_coordinate = y_coordinate;
    }

    fn is_touching(&self, ball: &Ball) -> bool {
        !self.get_hitbox().is_disjoint(&ball.get_hitbox())
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
        let mut player = Paddle::new_player(1);
        let mut rival = Paddle::new_rival(term_width, 1);
        let mut ball = Ball::new(5, 1);

        // Init game
        let mut start = Instant::now();
        draw_paddle(&mut screen, &player);
        draw_paddle(&mut screen, &rival);
        draw_ball(&mut screen, &ball);
        screen.flush().unwrap();

        loop {
            // Animation frames...
            let duration = start.elapsed();
            if duration >= Duration::from_millis(33) {
                start = Instant::now();

                ball.move_towards(term_width, term_height);

                write!(screen, "{}", termion::clear::All).unwrap();
                draw_paddle(&mut screen, &player);
                draw_paddle(&mut screen, &rival);
                draw_ball(&mut screen, &ball);
                screen.flush().unwrap();
            }
            // Game logic...
            ball.check_collision(&rival);
            ball.check_collision(&player);

            if let Some(c) = keys_pressed.next() {
                player.y_coordinate = match c.unwrap() {
                    Key::Char('q') => break,
                    Key::Char('z') => cmp::max(player.y_coordinate - 1, 1),
                    Key::Char('x') => cmp::min(player.y_coordinate + 1, term_height - 3),
                    Key::Null | _ => player.y_coordinate,
                };
            };
            // This handles frame skipping
            if player.is_touching(&ball) && (ball.x_coordinate == player.x_coordinate) {
                player.catch_ball(ball.y_coordinate)
            };

            if rival.y_coordinate % DIFICULTY == 0 {
                continue;
            } else {
                rival.catch_ball(ball.y_coordinate);
            };
        }
    }
}
