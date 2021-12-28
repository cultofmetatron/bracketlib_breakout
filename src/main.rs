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

const SCREEN_WIDTH: isize = 80;
const SCREEN_HEIGHT: isize = 50;
const FRAME_DURATION: f32 = 75.0;
const PADDLE_SIZE: isize = 5;

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
    fn move_right(&mut self, x: isize) {
        let new_position = if self.velocity.x >= 0 {
            self.velocity.x = self.velocity.x + x;
            self.x + self.velocity.x
        } else {
            self.velocity.x = 0;
            self.velocity.x = self.velocity.x + x;
            self.x + self.velocity.x
        };

        if new_position + PADDLE_SIZE < SCREEN_WIDTH {
            self.x = new_position;
        } else {
            self.x = SCREEN_WIDTH - PADDLE_SIZE;
        }
    }
    fn move_left(&mut self, x: isize) {
        let new_position = if self.velocity.x <= 0 {
            self.velocity.x = self.velocity.x - x;
            self.x + self.velocity.x
        } else {
            self.velocity.x = 0;
            self.velocity.x = self.velocity.x - x;
            self.x + self.velocity.x
        };

        if new_position > 0 {
            self.x = new_position;
        } else {
            self.x = 0;
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        for i in self.x..(self.x + PADDLE_SIZE) {
            ctx.set(
                i,
                SCREEN_HEIGHT - 2,
                BLACK,
                WHITE,
                to_cp437(' '),
            );
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
    fn set_velocity(&mut self, v: Velocity) {
        self.velocity = v;
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
    fn restart(&mut self) {
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0;
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over!");
        ctx.print_centered(6, &format!("You earned {} points.", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.paddle.render(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Q => ctx.quitting = true,
                VirtualKeyCode::Right => {
                    //if self.paddle.x + PADDLE_SIZE < SCREEN_WIDTH {
                    //    self.paddle = Paddle::new(self.paddle.x + 2);
                    //
                    self.paddle.move_right(1);
                }
                VirtualKeyCode::Left => {
                    //if self.paddle.x > 0 {
                    //    self.paddle = Paddle::new(self.paddle.x - 2);
                    //}
                    self.paddle.move_left(1);
                }
                _ => {}
            }
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Basic Breakout 2");
        ctx.print_centered(8, "(p) play game");
        ctx.print_centered(10, "(q) to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx)
        }
    }
}

fn main() -> BError {
    println!("Bootng Breakout");
    let context = BTermBuilder::simple80x50()
        .with_title("hello draggon")
        .build()?;

    main_loop(context, State::new())
}
