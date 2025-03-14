#![no_std]

use num::Integer;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{
    is_drawable, plot, plot_num, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH
};

use core::{
    clone::Clone,
    cmp::{min, Eq, PartialEq},
    iter::Iterator,
    marker::Copy,
    prelude::rust_2024::derive,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NewSnakeChar {
    x: usize,
    y: usize,
    character: char,
}



#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LetterMover {
    letters: [char; BUFFER_WIDTH],
    num_letters: usize,
    next_letter: usize,
    col: usize,
    row: usize,
    dx: isize,
    dy: isize,
    direction: Direction,
    x_points: [usize; 100],
    y_points: [usize; 100],
    points_used: usize,
    new_snake: NewSnakeChar,
    score: isize
, 
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}


pub fn safe_add<const LIMIT: usize>(a: usize, b: usize) -> usize {
    (a + b).mod_floor(&LIMIT)
}

pub fn add1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, 1)
}

pub fn sub1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, LIMIT - 1)
}


impl Default for LetterMover {
    fn default() -> Self {
        

        Self {
            letters: ['o'; BUFFER_WIDTH],
            num_letters: 1,
            next_letter: 1,
            col : BUFFER_WIDTH / 2,
            row: BUFFER_HEIGHT / 2,
            dx: 0,
            dy: 0,
            direction: Direction::Right,
            x_points: [10; 100],
            y_points: [10; 100],
            points_used: 1,
            new_snake: NewSnakeChar::new(),
            score: 0
        }
    }
}

impl NewSnakeChar {
    fn new() -> Self {
        Self {
            x: (BUFFER_WIDTH / 2)+10,
            y: (BUFFER_HEIGHT / 2)+10,
            character: 'o', 
        }
    }

    fn draw(&self) {
        plot(
            self.character,
            self.x,
            self.y,
            ColorCode::new(Color::Red, Color::Red),
        );
    }

    fn clear_current(&self) {
        plot(' ', self.x, self.y, ColorCode::new(Color::Black, Color::Black));
    }
}

impl LetterMover {
   // fn letter_columns(&self) -> impl Iterator<Item = usize> + '_ {
     //   (0..self.num_letters).map(|n| safe_add::<BUFFER_WIDTH>(n, self.col))
    //}


    pub fn tick(&mut self) {
        self.clear_current();
        self.update_location();
    
        if self.col == self.new_snake.x && self.row == self.new_snake.y {
            self.handle_unicode(self.new_snake.character);
            self.handle_add(self.new_snake.character,self.new_snake.x, self.new_snake.y);
            self.new_snake.clear_current();
            self.clear_food();
            self.new_food();
            self.score += 1;
            self.points_used += 1;


        }
    
        self.draw_current();
        self.new_snake.draw();
        
    }

    

    fn clear_current(&self) {
        plot(' ', self.col, self.row, ColorCode::new(Color::Black, Color::Black));
        
    }
    fn clear_food(&self) {
        plot(' ', self.new_snake.x, self.new_snake.y, ColorCode::new(Color::Black, Color::Black));
    }

    fn new_food(&mut self) {
        let seed = unsafe { core::arch::x86_64::_rdtsc() }; 
        let mut rng = oorandom::Rand32::new(seed);
        let new_x = rng.rand_range(0..(BUFFER_WIDTH as u32)) as usize;
        let new_y = rng.rand_range(0..(BUFFER_HEIGHT as u32)) as usize;
        self.clear_food();
        self.new_snake = NewSnakeChar {
            x: new_x,
            y: new_y,
            character: 'o',
        };
    }

    fn update_location(&mut self) {
        
    self.col = self.x_points[0];
    self.row = self.y_points[0];
    let new_x = self.x_points[0] as isize + self.dx;
    let new_y = self.y_points[0] as isize + self.dy;
    
    if 0 <= new_x && (new_x as usize) < BUFFER_WIDTH {
        self.x_points[0] = new_x as usize;
    }
    if 0 <= new_y && (new_y as usize) < BUFFER_HEIGHT{
        self.y_points[0] = new_y as usize;
    }
    }

    fn draw_current(&self) {
        for i in 0..self.points_used{
            plot(
                self.letters[i],
                self.x_points[i],
                self.y_points[i],
                ColorCode::new(Color::Green, Color::Green),
            );
        }
        plot_num(
            self.score,
            5,
            2,
            ColorCode::new(Color::White, Color::Black)
        );

    }


    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c),
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                if self.dx == 0{
                    self.dx = -1;
                    self.dy = 0;
                    self.direction = Direction::Left;
                    
                }
        
            }
            KeyCode::ArrowRight => {
                if self.dx == 0{
                    self.dx = 1;
                    self.dy = 0;
                    self.direction = Direction::Right;
                    
                }
                
            }
            KeyCode::ArrowUp => {
                if self.dy == 0{
                    self.dy = -1;
                    self.dx = 0;
                    self.direction = Direction::Up;
                    
                }
                
            }
            KeyCode::ArrowDown => {
                if self.dy == 0{
                    self.dy = 1;
                    self.dx = 0;
                    self.direction = Direction::Down;
                    
                }
                
            }
            _ => {}
        }
    }

    fn handle_add(&mut self, key: char,x: usize, y: usize) {
        if is_drawable(key) {
            self.x_points[self.next_letter-1] = x;
            self.y_points[self.next_letter-1] = y;
        }
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
            self.letters[self.next_letter] = key;
            self.next_letter = add1::<BUFFER_WIDTH>(self.next_letter);
            self.num_letters = min(self.num_letters + 1, BUFFER_WIDTH);
        }
    }
}
