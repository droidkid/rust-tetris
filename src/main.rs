extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

struct Pos2D {
    x: i32,
    y: i32,
}

impl Pos2D {
    fn xy(x:i32, y:i32) -> Self {
        Pos2D {
            x,
            y,
        }
    }
}

trait Drawable {
    fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos:Pos2D);
}

struct TetrisPiece {
    shape: [Pos2D; 4],

    // Drawing Related Constants
    color: Color,
}

impl TetrisPiece {
    fn build_stick_piece() -> Self {
        TetrisPiece {
            shape: [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(2,0), Pos2D::xy(3,0) ],
            color: Color::RGB(0, 255, 255),
        }
    }
    fn build_block_piece() -> Self {
        TetrisPiece {
            shape: [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(0,1) ],
            color: Color::RGB(255, 255, 0),
        }
    }

    fn build_t_piece() -> Self {
        TetrisPiece {
            shape: [ Pos2D::xy(1,0), Pos2D::xy(0,1), Pos2D::xy(1,1), Pos2D::xy(2,1) ],
            color: Color::RGB(128, 0, 128),
        }
    }

    fn build_s_piece() -> Self {
        TetrisPiece {
            shape: [ Pos2D::xy(1,0), Pos2D::xy(2,0), Pos2D::xy(1,1), Pos2D::xy(0,1) ],
            color: Color::RGB(0, 255, 0),
        }
    }

    fn build_z_piece() -> Self {
        TetrisPiece {
            shape: [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(2,1) ],
            color: Color::RGB(255, 0, 0),
        }
    }

    fn build_j_piece() -> Self {
        TetrisPiece {
            shape: [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(2,0), Pos2D::xy(2,1) ],
            color: Color::RGB(0, 0, 255),
        }
    }

    fn build_l_piece() -> Self {
        TetrisPiece {
            shape: [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(2,0), Pos2D::xy(0,1) ],
            color: Color::RGB(255, 165, 0),
        }
    }
}

impl Drawable for TetrisPiece {
    fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos:Pos2D) {
        let box_width = 20;
        canvas.set_draw_color(self.color);
        for diff in self.shape.iter() {
            let rect = Rect::new(
                pos.x + diff.x * box_width, 
                pos.y + diff.y * box_width, 
                box_width as u32, 
                box_width as u32
            );
            canvas.fill_rect(rect);
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust Tetris", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let stick_piece = TetrisPiece::build_stick_piece();
    let box_piece = TetrisPiece::build_block_piece();
    let t_piece = TetrisPiece::build_t_piece();
    let s_piece = TetrisPiece::build_s_piece();
    let z_piece = TetrisPiece::build_z_piece();
    let j_piece = TetrisPiece::build_j_piece();
    let l_piece = TetrisPiece::build_l_piece();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                }
                _ => {}
            }
        }
        stick_piece.draw(&mut canvas, Pos2D::xy(30, 50));
        box_piece.draw(&mut canvas, Pos2D::xy(100, 100));
        t_piece.draw(&mut canvas, Pos2D::xy(180, 100));
        s_piece.draw(&mut canvas, Pos2D::xy(280, 100));
        z_piece.draw(&mut canvas, Pos2D::xy(380, 100));
        j_piece.draw(&mut canvas, Pos2D::xy(180, 200));
        l_piece.draw(&mut canvas, Pos2D::xy(280, 200));

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }
}
