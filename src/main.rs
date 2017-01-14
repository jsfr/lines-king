extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

type Point = (u8, u8);
type Color = [f32; 4];

const RED:    Color = [1.0, 0.0, 0.0, 1.0];
const GREEN:  Color = [0.0, 1.0, 0.0, 1.0];
const BLUE:   Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: Color = [0.0, 1.0, 1.0, 1.0];
const BLACK:  Color = [0.0, 0.0, 0.0, 1.0];

const BOARD_WIDTH: u8 = 100;
const BOARD_HEIGHT: u8 = 100;
const TILE_SIZE: u8 = 4;
const UPDATE_TIME: f64 = 0.025;

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Occupied(Color)
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    width: u8,
    height: u8
}

struct Player {
    pos: Point,
    color: [f32; 4],
    direction: Direction,
    buttons: (Button, Button)
}

struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    players: Vec<Player>,
    board: Board,
    time: f64,
    update_time: f64,
    tile_size: u8
}

// fn mod(a, b) {
//     ((a % b) + b) % b
// }

impl Player {
    fn turn(&mut self, button: &Button) {
        if self.buttons.0 == *button {
            self.turn_left()
        } else if self.buttons.1 == *button {
            self.turn_right()
        }
    }

    fn turn_left(&mut self) {
        use Direction::*;
        match self.direction {
            North => self.direction = West,
            South => self.direction = East,
            East => self.direction = North,
            West => self.direction = South
        }
    }

    fn turn_right(&mut self) {
        use Direction::*;
        match self.direction {
            North => self.direction = East,
            South => self.direction = West,
            East => self.direction = South,
            West => self.direction = North
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::North => self.pos.1 -= 1,
            Direction::South => self.pos.1 += 1,
            Direction::West => self.pos.0 -= 1,
            Direction::East => self.pos.0 += 1,
        }
    }
}

impl Board {
    fn new(width: u8, height: u8, players: &Vec<Player>) -> Board {
        let mut tiles = vec![vec![Tile::Empty; height as usize]; width as usize];
        for player in players {
            tiles[player.pos.0 as usize][player.pos.1 as usize] = Tile::Occupied(player.color);
        }
        Board {
            width: width,
            height: height,
            tiles: tiles
        }
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let tile_size = self.tile_size as f64;
        let width = self.board.width;
        let height = self.board.height;
        let ref tiles = self.board.tiles;

        self.gl.draw(args.viewport(), |context, gl| {
            clear(BLACK, gl);
            // rectangle(color, player_square, context.transform, gl);
            for x in 0..width-1 {
                for y in 0..height-1 {
                    if let Tile::Occupied(color) = tiles[x as usize][y as usize] {
                        let square = rectangle::square(
                            x as f64 * tile_size,
                            y as f64 * tile_size,
                            tile_size
                            );
                        rectangle(color, square, context.transform, gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;

        while self.time > self.update_time {
            self.time -= self.update_time;
            for mut player in &mut self.players {
                player.step();
                self.board.tiles[player.pos.0 as usize][player.pos.1 as usize] = Tile::Occupied(player.color);
            }
        }
    }

    fn press_button(&mut self, button: &Button) {
        for mut player in &mut self.players {
            player.turn(button);
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let width = BOARD_WIDTH as u32 * TILE_SIZE as u32;
    let height = BOARD_HEIGHT as u32 * TILE_SIZE as u32;

    // Create a glutin window.
    let mut window: Window = WindowSettings::new("Lines King", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let players = vec![
        Player {
            pos: (1, 1),
            color: RED,
            direction: Direction::East,
            buttons: (Button::Keyboard(Key::A), Button::Keyboard(Key::D))
        },
        Player {
            pos: (10, 10),
            color: BLUE,
            direction: Direction::South,
            buttons: (Button::Keyboard(Key::D9), Button::Keyboard(Key::D0))
        }
    ];

    let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT, &players);


    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        players: players,
        board: board,
        time: 0.0,
        update_time: UPDATE_TIME,
        tile_size: TILE_SIZE
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press_button(&b)
        }

    }
}
