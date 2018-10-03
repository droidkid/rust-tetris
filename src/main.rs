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
    pos: Pos2D,
    shape: [[Pos2D; 4]; 4],
    orientation: usize, 
    // Drawing Related Constants
    color: Color,
}


impl TetrisPiece {
    fn build_stick_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape: [
                [ Pos2D::xy(-1,0), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(2,0) ],
                [ Pos2D::xy(1,-1), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(1,2) ],
                [ Pos2D::xy(-1,1), Pos2D::xy(0,1), Pos2D::xy(1,1), Pos2D::xy(2,1) ],
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(0,1), Pos2D::xy(0,2) ],
            ],
            color: Color::RGB(0, 255, 255),
            orientation: 0usize,
        }
    }

    fn build_block_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape: [
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(0,1) ],
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(2,0), Pos2D::xy(3,0) ],
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(2,0), Pos2D::xy(3,0) ],
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(2,0), Pos2D::xy(3,0) ],
            ],
            color: Color::RGB(255, 255, 0),
            orientation: 0usize,
        }
    }

    fn build_t_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape: [
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(-1,0) ],
                [ Pos2D::xy(1,0), Pos2D::xy(0,1), Pos2D::xy(0,-1), Pos2D::xy(0,0) ],
                [ Pos2D::xy(0,1), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(-1,0) ],
                [ Pos2D::xy(-1,0), Pos2D::xy(0,1), Pos2D::xy(0,-1), Pos2D::xy(0,0) ],
            ],
            color: Color::RGB(128, 0, 128),
            orientation: 0usize,
        }
    }

    fn build_s_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape: [
                [ Pos2D::xy(1,-1), Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(-1,0) ],
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1) ],
                [ Pos2D::xy(1,0), Pos2D::xy(0,0), Pos2D::xy(0,1), Pos2D::xy(-1,1) ],
                [ Pos2D::xy(-1,-1), Pos2D::xy(0,0), Pos2D::xy(-1,0), Pos2D::xy(0,1) ],
            ],

            color: Color::RGB(0, 255, 0),
            orientation: 0usize,
        }
    }

    fn build_z_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape: [ 
                [ Pos2D::xy(-1,-1), Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(1,0) ],
                [ Pos2D::xy(1,-1), Pos2D::xy(1,0), Pos2D::xy(0,0), Pos2D::xy(0,1) ],
                [ Pos2D::xy(-1,0), Pos2D::xy(0,0), Pos2D::xy(0,1), Pos2D::xy(1,1) ],
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(-1,0), Pos2D::xy(-1,1) ],
                ],
            color: Color::RGB(255, 0, 0),
            orientation: 0usize,
        }
    }

    fn build_j_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape:[ 
                [ Pos2D::xy(-1,0), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(-1,-1) ],
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(0,1), Pos2D::xy(1,-1) ],
                [ Pos2D::xy(-1,0), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1) ],
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(0,1), Pos2D::xy(-1,1) ],
            ],
            color: Color::RGB(0, 0, 255),
            orientation: 0usize,
        }
    }

    fn build_l_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape: [
                [ Pos2D::xy(-1,0), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,-1) ],
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(0,1), Pos2D::xy(1,1) ],
                [ Pos2D::xy(-1,0), Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(-1,1) ],
                [ Pos2D::xy(0,-1), Pos2D::xy(0,0), Pos2D::xy(0,1), Pos2D::xy(-1,-1) ],
            ],
            color: Color::RGB(255, 165, 0),
            orientation: 0usize,
        }
    }

    fn rotate(&mut self) {
        self.orientation = (self.orientation + 1) % 4;
    }
}

impl Drawable for TetrisPiece {
    fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos:Pos2D) {
        let box_width = 20;
        canvas.set_draw_color(self.color);
        for diff in self.shape[self.orientation].iter() {
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
    let width = 200;
    let height = 400;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust Tetris", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut stick_piece = TetrisPiece::build_stick_piece(Pos2D::xy(0,0));
    let box_piece = TetrisPiece::build_block_piece(Pos2D::xy(0,0));
    let mut t_piece = TetrisPiece::build_t_piece(Pos2D::xy(0,0));
    let mut s_piece = TetrisPiece::build_s_piece(Pos2D::xy(0,0));
    let mut z_piece = TetrisPiece::build_z_piece(Pos2D::xy(0,0));
    let mut j_piece = TetrisPiece::build_j_piece(Pos2D::xy(0,0));
    let mut l_piece = TetrisPiece::build_l_piece(Pos2D::xy(0,0));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.fill_rect(Rect::new(0,0,width,height));

        stick_piece.draw(&mut canvas, Pos2D::xy(30, 50));
        box_piece.draw(&mut canvas, Pos2D::xy(150, 100));
        t_piece.draw(&mut canvas, Pos2D::xy(80, 100));
        s_piece.draw(&mut canvas, Pos2D::xy(80, 180));
        z_piece.draw(&mut canvas, Pos2D::xy(20, 180));
        j_piece.draw(&mut canvas, Pos2D::xy(80, 320));
        l_piece.draw(&mut canvas, Pos2D::xy(80, 240));

        stick_piece.rotate();
        t_piece.rotate();
        s_piece.rotate();
        z_piece.rotate();
        j_piece.rotate();
        l_piece.rotate();

        canvas.present();
        thread::sleep(Duration::new(2, 0));
    }
}
