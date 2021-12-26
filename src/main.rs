use std::io::stdin;
use bracket_lib::prelude::*;


/*

a basic breakout game based on code for the flappy bird from hands on rust

milestone 1:
    * render the paddle and ball
    * left and right arrows should move the paddle around

milestone 2:
    * render the ball should move around and bounce off the walls and paddle.
      if it hits the floor, game over

milestone 3:
    * render bricks, ball should bounce off the bricks

miles


*/

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Playing,
    Menu,
    End,
}

struct Paddle {
    x: isize,
    velocity: Velocity,
}

impl Paddle {
    fn new(x: isize) -> Self {
        Self {
            x,
            velocity: Velocity {
                x: 0,
                y: 0,
            },
        }
    }
}

struct Brick {
    x: isize,
    y: isize,
    size: isize,
}


struct Velocity {
    x: isize,
    y: isize,
}

struct Ball {
    x: isize,
    y: isize,
    velocity: Velocity,
}

impl Ball {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            velocity: Velocity { x: 0, y: 0 },
        }
    }
}

struct State {
    mode: GameMode,
    frame_time: f32,
    score: isize,
    paddle: Paddle,
    ball: Ball,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,
            paddle: Paddle::new(0),
            ball: Ball::new(10, 10),
            score: 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
