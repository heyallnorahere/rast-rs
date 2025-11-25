use std::convert::From;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for RGBA8 {
    fn default() -> RGBA8 {
        RGBA8 {
            r: 0,
            g: 0,
            b: 0,
            a: 1,
        }
    }
}

impl Index<usize> for RGBA8 {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Invalid color component!"),
        }
    }
}

impl IndexMut<usize> for RGBA8 {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("Invalid color component!"),
        }
    }
}

fn color_bit_offset(channel: usize, count: usize) -> usize {
    let byte_index = count - (channel + 1);
    byte_index * 8
}

impl From<u32> for RGBA8 {
    fn from(value: u32) -> RGBA8 {
        let mut result = RGBA8::default();

        for i in 0..4 {
            let bit_offset = color_bit_offset(i, 4);
            result[i] = ((value >> bit_offset) & 0xFF) as u8;
        }

        result
    }
}

impl Into<u32> for RGBA8 {
    fn into(self) -> u32 {
        let mut result = 0;

        for i in 0..4 {
            let bit_offset = color_bit_offset(i, 4);
            result |= (self[i] as u32) << bit_offset;
        }

        result
    }
}
