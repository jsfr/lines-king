extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const RED:     [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:    [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const YELLOW:  [f32; 4] = [0.0, 1.0, 1.0, 1.0];
const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

type Point = (u8, u8);

enum Direction {
    North,
    East,
    South,
    West
}

struct Board {
    tiles: Vec<Vec<bool>>,
    width: u8,
    height: u8
}

struct Player {
    pos: Point,
    color: [f32; 4],
    direction: Direction
}

struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player,
    board: Board,
    time: f64,
    update_time: f64,
    tile_size: u8
}

impl Board {
    fn new(width: u8, height: u8) -> Board {
        let mut tiles = Vec::new();
        for _ in 0..width-1 {
            tiles.push(vec![false; height as usize]);
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
        use graphics::*;

        let x = self.player.pos.0 as f64;
        let y = self.player.pos.1 as f64;
        let color = self.player.color;
        let tile_size = self.tile_size as f64;
        let player_square = rectangle::square(x * tile_size, y * tile_size, tile_size);
        let width = self.board.width;
        let height = self.board.height;
        let ref tiles = self.board.tiles;


        self.gl.draw(args.viewport(), |context, gl| {
            clear(BLACK, gl);
            rectangle(color, player_square, context.transform, gl);
            for x in 0..width-1 {
                for y in 0..height-1 {
                    if tiles[x as usize][y as usize] {
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

            self.board.tiles[self.player.pos.0 as usize][self.player.pos.1 as usize] = true;

            match self.player.direction {
                Direction::North => self.player.pos.1 -= 1,
                Direction::South => self.player.pos.1 += 1,
                Direction::West => self.player.pos.0 -= 1,
                Direction::East => self.player.pos.0 += 1,
            }
        }
    }
}


const BOARD_WIDTH: u8 = 50;
const BOARD_HEIGHT: u8 = 50;
const TILE_SIZE: u8 = 10;
const UPDATE_TIME: f64 = 0.10;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let width = BOARD_WIDTH as u32 * TILE_SIZE as u32;
    let height = BOARD_HEIGHT as u32 * TILE_SIZE as u32;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Lines King", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let player = Player {
        pos: (1, 3),
        color: RED,
        direction: Direction::East
    };

    let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: player,
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
    }
}
