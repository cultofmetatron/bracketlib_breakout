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
const FRAME_DURATION: f32 = 20.0;
const PADDLE_SIZE: isize = 5;
const PADDLE_Y: isize = SCREEN_HEIGHT - 2;

enum GameMode {
    Playing,
    Menu,
    End,
}

#[derive(Clone, Copy, Debug)]
struct Boundry {
    position: (isize, isize),
    glyph: FontCharType,
}


impl Boundry {
    fn new(x: isize, y: isize, glyph: char) -> Self {
        Boundry {
            position: (x, y),
            glyph: to_cp437(glyph),
        }
    }
    fn detect_collision(self, ball: Ball) -> bool {
        let (ball_x, ball_y) = ball.next_position();
        let (x, y) = self.position;
        (x == ball_x && y == ball_y)
    }
    fn render(self, ctx: &mut BTerm) {
        let (x, y) = self.position;
        ctx.set(x, y, BLACK, WHITE, self.glyph);
    }
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
    fn detect_collision(self, ball: Ball) -> bool {
        let (ball_x, ball_y) = ball.next_position();
        let x = self.x;
        (x == ball_x && ball_y == PADDLE_Y)
    }
    fn render(&mut self, ctx: &mut BTerm) {
        for i in self.x..(self.x + PADDLE_SIZE) {
            ctx.set(
                i,
                PADDLE_Y,
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
    launched: bool
}

impl Ball {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            velocity: Velocity { x: 0, y: 0 },
            launched: false,
        }
    }
    fn set_position(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }
    fn set_velocity(&mut self, v: Velocity) {
        self.velocity = v;
    }
    fn update_position(&mut self) {
        self.x = self.x + self.velocity.x;
        self.y = self.y + self.velocity.y;
    }
    // gets the next position of the ball
    fn next_position(self) -> (isize, isize) {
        (self.x + self.velocity.x, self.y + self.velocity.y)
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.x,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@'),
        );
    }
}

struct State {
    mode: GameMode,
    frame_time: f32,
    score: isize,
    paddle: Paddle,
    ball: Ball,
    wall_tiles: Vec<Boundry>
}

impl State {
    fn new() -> Self {
        let tiles = Self::init_wall_tiles();

        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,
            paddle: Paddle::new(0),
            ball: Ball::new(10, 10),
            score: 0,
            wall_tiles: tiles,
        }
    }
    fn init_wall_tiles() -> Vec<Boundry> {
        let mut tiles: Vec<Boundry> = vec![];
        // for the top
        for i in 1..(SCREEN_WIDTH - 1) {
            tiles.push(Boundry::new(i, 2, '_'));
        }

        tiles
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

        for wall in self.wall_tiles.iter() {
            wall.render(ctx);
        }

        if self.ball.launched {
            self.ball.update_position();
            self.ball.render(ctx);
        } else {
            self.ball.set_position(self.paddle.x, PADDLE_Y - 1);
            self.ball.render(ctx);
        }
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
                VirtualKeyCode::Space => {
                    if !self.ball.launched {
                        self.ball.launched = true;
                        let ball_velocity = if self.paddle.velocity.x >= 0 {
                            1
                        } else {
                            -1
                        };
                        self.ball.set_velocity(Velocity { x: ball_velocity, y: -1 });
                    }
                }
                _ => {
                    self.paddle.velocity = Velocity { x: 0, y: 0 }
                }
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
