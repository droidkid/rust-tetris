extern crate sdl2;
extern crate rand;


use rand::{thread_rng,Rng};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::render::TextureQuery;

use std::mem;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};


// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);


static start_pos: Pos2D = Pos2D {
    x: 5,
    y: 1
};

#[derive(Copy, Clone)]
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

    fn add(&mut self, pos: Pos2D) {
        self.x = self.x + pos.x;
        self.y = self.y + pos.y;
    }

    fn inv(&self) -> Self {
        Pos2D {
            x: -self.x,
            y: -self.y,
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
    fn build_i_piece(pos: Pos2D) -> Self {
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

    fn build_o_piece(pos: Pos2D) -> Self {
        TetrisPiece {
            pos: pos,
            shape: [
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(0,1) ],
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(0,1) ],
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(0,1) ],
                [ Pos2D::xy(0,0), Pos2D::xy(1,0), Pos2D::xy(1,1), Pos2D::xy(0,1) ],
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

    fn move_by(&mut self, pos: Pos2D) {
        self.pos.add(pos);
    }

    fn rotate_right(&mut self) {
        self.orientation = (self.orientation + 1) % 4;
    }
    fn rotate_left(&mut self) {
        self.orientation = (self.orientation + 3) % 4;
    }

    fn iter(&self) -> TetrisPieceIter {
        TetrisPieceIter {
            block_num: 0usize,
            piece: &self,
        }
    }
}

struct RandomTetrisPieceGenerator {
    piece_seq: Vec<i32>,
    idx: usize,
}

impl RandomTetrisPieceGenerator {
    fn new() -> Self {
        RandomTetrisPieceGenerator {
            piece_seq: RandomTetrisPieceGenerator::next_permut(),
            idx: 0usize,
        }
    }
    
    fn next_permut() -> Vec<i32> {
        let mut piece_seq: Vec<i32> = (0..7).collect();
        {
            let slice: &mut [i32] = &mut piece_seq;
            thread_rng().shuffle(slice);
        }
        piece_seq
    }

    fn get_next_piece(&mut self, pos: Pos2D) -> TetrisPiece {
        self.idx = (self.idx + 1) % 7;
        if self.idx == 0 {
            self.piece_seq = RandomTetrisPieceGenerator::next_permut();
        }
        self.get_piece_for_num(self.piece_seq[self.idx], pos).unwrap()
    }

    fn get_piece_for_num(&self, num: i32, pos: Pos2D) -> Option<TetrisPiece> {
        match num {
            0 => Some(TetrisPiece::build_i_piece(pos)),
            1 => Some(TetrisPiece::build_o_piece(pos)),
            2 => Some(TetrisPiece::build_s_piece(pos)),
            3 => Some(TetrisPiece::build_z_piece(pos)),
            4 => Some(TetrisPiece::build_j_piece(pos)),
            5 => Some(TetrisPiece::build_l_piece(pos)),
            6 => Some(TetrisPiece::build_t_piece(pos)),
            _ => None
        }
    }
}

struct TetrisPieceIter<'a> {
    block_num: usize,
    piece: &'a TetrisPiece,
}

impl<'a> Iterator for TetrisPieceIter<'a> {
    type Item = Pos2D;

    fn next(&mut self) -> Option<Pos2D> {
        if self.block_num == 4 {
            return None;
        }
        let diff = &self.piece.shape[self.piece.orientation][self.block_num];
        let mut pos = Pos2D::xy(self.piece.pos.x, self.piece.pos.y);

        pos.x = pos.x + diff.x;
        pos.y = pos.y + diff.y;

        self.block_num = self.block_num + 1;

        Some(pos)
    }
}


impl Drawable for TetrisPiece {
    fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos:Pos2D) {
        let box_width = 20;
        canvas.set_draw_color(self.color);
        for diff in self.shape[self.orientation].iter() {
            let rect = Rect::new(
                (pos.x + diff.x * box_width) + 1, 
                (pos.y + diff.y * box_width) + 1, 
                (box_width - 2) as u32, 
                (box_width - 2) as u32
            );
            canvas.fill_rect(rect);
        }
    }
}

struct TetrisUnitBlock {
    is_filled: bool,
    color: Color,
}

struct TetrisBoard {
    width: usize,
    height: usize,
    board: Vec<Vec<TetrisUnitBlock>>,
    active_piece: TetrisPiece,
    tetris_gen: RandomTetrisPieceGenerator,
    gravity: u32,
    gravity_countdown: u32,
    lock_delay: u32,
    lock_delay_countdown: u32,
    lines_cleared: u32
}

struct Input {
    left_key_pressed: bool,
    right_key_pressed: bool,
    up_key_pressed:bool,
    down_key_pressed:bool,
}

impl Input {
    fn reset(&mut self) {
        self.left_key_pressed = false;
        self.right_key_pressed = false;
        self.up_key_pressed = false;
        self.down_key_pressed = false;
    }
}


impl TetrisBoard {

    fn new() -> Self {

        let mut board: Vec<Vec<TetrisUnitBlock>> = Vec::new();
        let width: usize = 12;
        let height: usize = 24;

        for i in 0usize..height {
            board.push(Vec::new());
            for _ in 0usize..width {
                board[i].push(TetrisUnitBlock { is_filled: false, color: Color::RGB(0,0,0) } );
            }
        }

        for i in 0usize..width {
            board[0][i] = TetrisUnitBlock { is_filled:true, color: Color::RGB(255,255,255) };
            board[height-1][i] = TetrisUnitBlock { is_filled:true, color: Color::RGB(255,255,255) };
        }
        for i in 0usize..height {
            board[i][0] = TetrisUnitBlock { is_filled:true, color: Color::RGB(255,255,255) };
            board[i][width-1] = TetrisUnitBlock { is_filled:true, color: Color::RGB(255,255,255) };
        }

        let mut randomTetrisPieceGenerator = RandomTetrisPieceGenerator::new();

        TetrisBoard {
            width: width,
            height: height,
            board: board,
            active_piece: randomTetrisPieceGenerator.get_next_piece(start_pos),
            tetris_gen: randomTetrisPieceGenerator,
            gravity: 20,
            gravity_countdown:  20,
            lock_delay: 30,
            lock_delay_countdown: 30,
            lines_cleared: 0,
        }
    }

    fn is_valid(&self) -> bool {
        for pos in self.active_piece.iter() {
            if self.board[pos.y as usize][pos.x as usize].is_filled {
                return false;
            }
        }
        true
    }

    fn is_game_over(&self) -> bool {
        for pos in self.active_piece.iter() {
            if pos.y == 0 {
                return true;
            }
        }
        false
    }

    fn move_active_piece(&mut self, pos: Pos2D) -> bool {
        self.active_piece.move_by(pos);

        if !self.is_valid() {
            self.active_piece.move_by(pos.inv());
            false
        } else {
            true
        }
    }

    fn rotate_active_piece_right(&mut self) -> bool {
        self.active_piece.rotate_right();

        if !self.is_valid() {
            self.active_piece.rotate_left();
            false
        } else {
            true
        }

    }

    fn consume(&mut self, piece: TetrisPiece) {
        for pos in piece.iter() {
            self.board[pos.y as usize][pos.x as usize].is_filled = true;
            self.board[pos.y as usize][pos.x as usize].color = piece.color;
        }
    }

    fn is_row_full(&self, row: usize) -> bool {
        for i in 1..self.width-1 {
            if !self.board[row][i].is_filled {
                return false;
            }
        }
        true
    }

    fn is_row_empty(&self, row:usize) -> bool {
        for i in 0..self.width-1 {
            if self.board[row][i].is_filled {
                return false;
            }
        }
        true
    }

    fn shift_down(&mut self, row:usize) {
        for i in (2..row+1).rev() {
            for j in (1..self.width-1) {
                self.board[i][j].is_filled = self.board[i-1][j].is_filled;
                self.board[i][j].color = self.board[i-1][j].color;
            }
        }
    }

    fn clear_lines(&mut self) {
        for i in (1..self.height-1).rev() {
            while self.is_row_full(i) {
                self.lines_cleared += 1;
                self.shift_down(i);
            }
        }
    }



    fn update(&mut self, input: &Input) {

        // Handle Input
        if (input.left_key_pressed) {
            self.move_active_piece(Pos2D::xy(-1, 0));
        } 
        if (input.right_key_pressed) {
            self.move_active_piece(Pos2D::xy(1, 0));
        }
        if (input.up_key_pressed) {
            self.rotate_active_piece_right();
        }
        if (input.down_key_pressed) {
            self.gravity = 5;
        } else {
            self.gravity = 20;
        }

        if (self.gravity_countdown > 0) {
            self.gravity_countdown -= 1;
        }
        if (self.lock_delay_countdown > 0) {
            self.lock_delay_countdown -= 1;
        }


        let mut move_down_success = true;

        if self.gravity_countdown == 0 {
            self.gravity_countdown = self.gravity;
            move_down_success = self.move_active_piece(Pos2D::xy(0,1));
            if move_down_success {
                self.lock_delay_countdown= self.lock_delay;
            }
        }

        if !move_down_success {

            if self.lock_delay_countdown == 0 {
                self.lock_delay_countdown = self.lock_delay;
                if (self.is_game_over()) {
                    return;
                } 
                else {
                    let piece_to_consume = mem::replace(&mut self.active_piece, self.tetris_gen.get_next_piece(start_pos));
                    self.consume(piece_to_consume);
                    self.clear_lines();
                }
            }

        }
    }
}

impl Drawable for TetrisUnitBlock {
    fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos:Pos2D) {
        let box_width = 20;
        canvas.set_draw_color(self.color);
        let rect = Rect::new(
                pos.x + 1,  
                pos.y + 1, 
                box_width - 2 as u32, 
                box_width - 2 as u32
        );
        canvas.fill_rect(rect);
    }
}

impl Drawable for TetrisBoard {
    fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos:Pos2D) {
        let box_width: i32 = 20;
        for i in 0usize..self.board.len() {
            for j in 0usize..self.board[i].len() {
                let x:i32 = (j as i32) * box_width + pos.x;
                let y:i32 = (i as i32) * box_width + pos.y;
                self.board[i][j].draw(canvas, Pos2D::xy(x, y))
            }
        }

        let x = self.active_piece.pos.x * box_width + pos.x;
        let y = self.active_piece.pos.y * box_width + pos.y;
        self.active_piece.draw(canvas, Pos2D::xy(x,y));
    }
}



fn main() {
    let width = 800;
    let height = 600;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window("Rust Tetris", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let font_path: &Path = Path::new("res/fonts/kenney_future.ttf"); 
    let mut font = ttf_context.load_font(font_path, 28).unwrap();

    let tetris_surface = font.render("Tetris").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
    let tetris_texture = texture_creator.create_texture_from_surface(&tetris_surface).unwrap();
    let TextureQuery { width: text_width, height: text_height, .. } = tetris_texture.query();
    let tetris_target_rect = Rect::new(300, 10, text_width, text_height);


    let mut tetris_board = TetrisBoard::new();
    let mut last_updated = Instant::now();


    let mut input : Input = Input {
            left_key_pressed: false,
            right_key_pressed: false,
            up_key_pressed: false,
            down_key_pressed: false,
    };

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                }
                Event::KeyDown {keycode: Some(Keycode::Left), ..}  => {
                    input.left_key_pressed = true;
                }
                Event::KeyDown {keycode: Some(Keycode::Right), ..}  => {
                    input.right_key_pressed = true;
                }
                Event::KeyDown {keycode: Some(Keycode::Up), ..}  => {
                    input.up_key_pressed = true;
                }
                _ => {}
            }
        }

        if (event_pump.keyboard_state().is_scancode_pressed(Scancode::Down)) {
            input.down_key_pressed = true;
        }
        else {
            input.down_key_pressed = false;
        }

        let current_time = Instant::now();

        if current_time.duration_since(last_updated) > Duration::new(0, 1_000_000_000 / 60) {
            tetris_board.update(&input);
            input.reset();
            last_updated = current_time;
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.fill_rect(Rect::new(0,0,width,height));

        tetris_board.draw(&mut canvas, Pos2D::xy(250,50));
        {
            let score_surface = font.render(&format!("Lines : {}", tetris_board.lines_cleared))
                                            .blended(Color::RGBA(255, 255, 255, 255)).unwrap();
            let score_texture = texture_creator.create_texture_from_surface(&score_surface).unwrap();
            let TextureQuery { width: score_width, height: score_height, .. } = score_texture.query();
            let score_target_rect = Rect::new(500, 100, score_width/2 , score_height/2);
            canvas.copy(&score_texture, None, Some(score_target_rect)).unwrap();
        }

         canvas.copy(&tetris_texture, None, Some(tetris_target_rect)).unwrap();

        canvas.present();
    }
}
