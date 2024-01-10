use core::ops::{Index, IndexMut};
use nalgebra::Vector3;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::*;
use crate::color_to_vector;

pub struct WrappingVec {
    v: [Vector3<i32>; 1024],
    cursor: usize,
    size: usize,
}

impl WrappingVec {
    pub fn new<I>(size: Size, source_pixels: &mut I) -> Self where I: Iterator<Item = Rgb888> {
        let mut v = [Vector3::<i32>::default(); 1024];
        let size: usize = size.width as usize + 1;

        for item in v.iter_mut().take(size) {
            *item = color_to_vector(source_pixels.next().unwrap());
        }

        Self { v, cursor: 0, size }
    }

    pub fn push(&mut self, item: Vector3<i32>) {
        self.v[self.cursor] = item;
        self.cursor = (self.cursor + 1) % self.size;
    }
}

impl Index<usize> for WrappingVec {
    type Output = Vector3<i32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[(self.cursor + index) % self.size]
    }
}

impl IndexMut<usize> for WrappingVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(self.cursor + index) % self.size]
    }
}
