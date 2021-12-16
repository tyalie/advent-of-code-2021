extern crate alloc;

use core::cmp::min;
use alloc::vec::Vec;
use alloc::vec;


pub struct CostField {
    field: &'static mut [[u16; 400]; 400],
    top_left_corner: (u16, u16),
    size: (u16, u16)
}


impl CostField {
    pub fn new(size: &(u16, u16), memory: &'static mut [[u16; 400]; 400]) -> Self {
        Self::clear_mem(memory);
        CostField {
            field: memory,
            top_left_corner: (0, 0),
            size: *size
        }
    }

    fn clear_mem(mem: &mut [[u16; 400]; 400]) {
        for row in mem.iter_mut() {
            for v in row.iter_mut() {
                *v = u16::MAX;
            }
        }
    }

    fn rows(&self) -> u16 { 
        min(self.size.0, self.field.len() as u16)
    }
    fn cols(&self) -> u16 { 
        min(self.size.1, self.field[0].len() as u16)
    }

    pub fn move_field(&mut self, bottom_right: &(u16, u16)) {
        let move_y = if bottom_right.0 >= self.rows() + self.top_left_corner.0 {
            (bottom_right.0 - self.rows() - self.top_left_corner.0) as usize
        } else { 0 };

        let move_x = if bottom_right.1 >= self.rows() + self.top_left_corner.1 {
            (bottom_right.1 - self.rows() - self.top_left_corner.1) as usize
        } else { 0 };

        // can only move towards bottom right, never back
        if move_y == 0 && move_x == 0 { return; }

        for y in 0..(self.rows() as usize - move_y) {
            for x in 0..(self.cols() as usize - move_x) {
                self.field[y][x] = self.field[y + move_y][x + move_x];
                self.field[y + move_y][x + move_x] = 0;
            }
        }
    }

    fn bound_check(&self, coord: &(u16, u16)) -> Option<()> {
        if coord.0 < self.top_left_corner.0 || coord.1 < self.top_left_corner.1 
            || coord.0 >= self.rows() || coord.1 >= self.cols() {
            None
        } else {
            Some(())
        }
    }

    pub fn get(&self, coord: (u16, u16)) -> Option<&u16> {
        self.bound_check(&coord)?;
        self.field.get((coord.0 - self.top_left_corner.0) as usize)?
            .get((coord.1 - self.top_left_corner.1) as usize)
    }

    pub fn get_mut(&mut self, coord: (u16, u16)) -> Option<&mut u16> {
        self.bound_check(&coord)?;
        self.field.get_mut((coord.0 - self.top_left_corner.0) as usize)?
            .get_mut((coord.1 - self.top_left_corner.1) as usize)
    }

}
