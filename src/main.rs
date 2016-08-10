extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const SQUARE_SIZE: u32 = 5;
const PLAYER_SPEED: f64 = 100.0;
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

enum Direction {
    North,
    East,
    South,
    West
}

struct Player {
    location: (f64, f64),
    color: [f32; 4],
    direction: Direction
}

struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let (x, y) = self.player.location;
        let color = self.player.color;
        let player_square = graphics::rectangle::square(
            x,
            y,
            SQUARE_SIZE as f64
        );

        self.gl.draw(args.viewport(), |context, gl| {
            graphics::rectangle(color, player_square, context.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let delta = PLAYER_SPEED * args.dt;

        match self.player.direction {
            Direction::North => self.player.location.1 -= delta,
            Direction::South => self.player.location.1 += delta,
            Direction::West => self.player.location.0 -= delta,
            Direction::East => self.player.location.0 += delta,
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Lines King",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let player = Player {
        location: (50.0, 50.0),
        color: RED,
        direction: Direction::East
    };

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: player
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
