use std::{fmt::Display, ops::Range};

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;
const SPRITE_SIZE: i32 = 3;

pub struct CRT {
    pixels: [[char; CRT_WIDTH]; CRT_HEIGHT],
}

impl CRT {
    pub fn new() -> Self {
        Self {
            pixels: [['.'; CRT_WIDTH]; CRT_HEIGHT],
        }
    }

    pub fn draw(&mut self, cpu_cycle: u32, register_x: i32) -> () {
        let row = Self::row(cpu_cycle);
        let column = Self::column(cpu_cycle);

        let sprite = Self::sprite_position(register_x);

        let pixel = Self::pixel(sprite, column);

        self.set_pixel(row, column, pixel)
    }

    fn set_pixel(&mut self, row: usize, column: usize, pixel: char) -> () {
        self.pixels[row][column] = pixel;
    }

    fn pixel_index(cycle: u32) -> usize {
        (cycle - 1) as usize
    }

    fn row(cycle: u32) -> usize {
        Self::pixel_index(cycle) / CRT_WIDTH
    }

    fn column(cycle: u32) -> usize {
        Self::pixel_index(cycle) % CRT_WIDTH
    }

    fn pixel(sprite: Range<i32>, column: usize) -> char {
        let is_lit_pixel = sprite.contains(&(column as i32));

        if is_lit_pixel {
            '#'
        } else {
            '.'
        }
    }

    fn sprite_position(register_x: i32) -> Range<i32> {
        let sprite_start_index = register_x - 1;

        sprite_start_index..(sprite_start_index + SPRITE_SIZE)
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;

        for row in self.pixels {
            for pixel in row {
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
