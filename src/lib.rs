#![no_std]

use embedded_graphics_core::pixelcolor::{Rgb888, RgbColor};
use nalgebra::Vector3;
mod wrapping_vec;

pub struct Dither<I, F, const WIDTH: usize, const WIDTH_PLUS_ONE: usize>
where
    I: Iterator<Item = Vector3<i16>>,
    F: Fn(Rgb888) -> Rgb888,
{
    buffer: wrapping_vec::WrappingVec<Vector3<i16>, WIDTH_PLUS_ONE>,
    vectors: I,
    closest_color: F,
}

impl<I, F, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> Dither<I, F, WIDTH, WIDTH_PLUS_ONE>
where
    I: Iterator<Item = Vector3<i16>>,
    F: Fn(Rgb888) -> Rgb888,
{
    pub fn new(mut vectors: I, closest_color: F) -> Self {
        Self {
            buffer: wrapping_vec::WrappingVec::new(&mut vectors),
            closest_color,
            vectors,
        }
    }
}

impl<I, F, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> Iterator for Dither<I, F, WIDTH, WIDTH_PLUS_ONE>
where
    I: Iterator<Item = Vector3<i16>>,
    F: Fn(Rgb888) -> Rgb888,
{
    type Item = Rgb888;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_pixel) = self.vectors.next() {
            let lookup = Rgb888::new(
                self.buffer[0][0].clamp(0, 255).try_into().unwrap_or(0x00),
                self.buffer[0][1].clamp(0, 255).try_into().unwrap_or(0x00),
                self.buffer[0][2].clamp(0, 255).try_into().unwrap_or(0x00),
            );
            let newpixel = (self.closest_color)(lookup);
            let quant_error = self.buffer[0] - Vector3::<i16>::new(
                newpixel.r().into(),
                newpixel.g().into(),
                newpixel.b().into()
            );

            self.buffer[1] += (quant_error * 7) / 16;
            self.buffer[WIDTH - 1] += (quant_error * 3) / 16;
            self.buffer[WIDTH] += (quant_error * 5) / 16;
            self.buffer[WIDTH + 1] += quant_error / 16;

            self.buffer.push(next_pixel);
            Some(newpixel)
        } else {
            None
        }
    }
}
