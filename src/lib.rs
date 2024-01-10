#![no_std]

use embedded_graphics_core::{
    geometry::Size,
    pixelcolor::{Rgb888, RgbColor},
    prelude::*,
};
use nalgebra::Vector3;
mod wrapping_vec;

pub fn color_to_vector<C>(color: C) -> Vector3<i32>
where
    C: RgbColor,
{
    Vector3::<i32>::new(color.r() as i32, color.g() as i32, color.b() as i32)
}

// eliminate panic
pub fn vector_to_color(color: Vector3<i32>) -> Rgb888 {
    Rgb888::new(
        color[0].clamp(0, 255).try_into().unwrap_or(0x00),
        color[1].clamp(0, 255).try_into().unwrap_or(0x00),
        color[2].clamp(0, 255).try_into().unwrap_or(0x00),
    )
}

pub struct Dither<I, F>
where
    I: Iterator<Item = Rgb888>,
    F: Fn(Rgb888) -> Rgb888,
{
    buffer: wrapping_vec::WrappingVec,
    size: Size,
    source_pixels: I,
    f: F,
}

impl<I, F> Dither<I, F>
where
    I: Iterator<Item = Rgb888>,
    F: Fn(Rgb888) -> Rgb888,
{
    // should error
    pub fn new(size: Size, mut source_pixels: I, f: F) -> Self {
        let v = wrapping_vec::WrappingVec::new(size, &mut source_pixels);

        Self {
            buffer: v,
            size,
            f,
            source_pixels,
        }
    }
}

impl<I, F> Iterator for Dither<I, F>
where
    I: Iterator<Item = Rgb888>,
    F: Fn(Rgb888) -> Rgb888,
{
    type Item = Rgb888;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_pixel) = self.source_pixels.next() {
            let oldpixel = self.buffer[0];
            let newpixel = (self.f)(vector_to_color(oldpixel));
            let quant_error = oldpixel - color_to_vector(newpixel);

            self.buffer[1] += (quant_error * 7) / 16;
            self.buffer[self.size.width as usize - 1] += (quant_error * 3) / 16;
            self.buffer[self.size.width as usize] += (quant_error * 5) / 16;
            self.buffer[self.size.width as usize + 1] += quant_error / 16;

            self.buffer.push(color_to_vector(next_pixel));

            Some(newpixel)
        } else {
            None
        }
    }
}

impl<I, F> OriginDimensions for Dither<I, F>
where
    I: Iterator<Item = Rgb888>,
    F: Fn(Rgb888) -> Rgb888,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[test]
fn test_dither() {
    let mut image: [Rgb888; 9] = vec![
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
        Rgb888::new(0xcc, 0xcc, 0xcc),
    ];
    let output: [Rgb888; 9] = dither(image.into_iter(), Size::new(3, 3), closest).collect();

    assert_eq!(
        output,
        [
            Rgb888::new(170, 170, 170),
            Rgb888::new(170, 170, 170),
            Rgb888::new(170, 170, 170),
            Rgb888::new(170, 170, 170),
            Rgb888::new(170, 170, 170),
            Rgb888::new(0, 0, 0),
            Rgb888::new(170, 170, 170),
            Rgb888::new(0, 0, 0),
            Rgb888::new(0, 0, 0)
        ]
    )
}

const CGA_PALETTE: [Rgb888; 14] = [
    Rgb888::new(0x00, 0x00, 0x00),
    Rgb888::new(0x00, 0x00, 0xAA),
    Rgb888::new(0x00, 0xAA, 0x00),
    Rgb888::new(0x00, 0xAA, 0xAA),
    Rgb888::new(0xAA, 0x00, 0x00),
    Rgb888::new(0xAA, 0x00, 0xAA),
    Rgb888::new(0xAA, 0xAA, 0xAA),
    Rgb888::new(0xAA, 0x55, 0x00),
    Rgb888::new(0xff, 0xff, 0xff),
    Rgb888::new(0x55, 0x55, 0x55),
    Rgb888::new(0x55, 0x55, 0xFF),
    Rgb888::new(0xFF, 0xFF, 0x55),
    Rgb888::new(0xFF, 0x55, 0x55),
    Rgb888::new(0x55, 0xFF, 0x55),
];

// eliminate panic
pub fn closest(pixel: Rgb888) -> Rgb888 {
    CGA_PALETTE
        .into_iter()
        .min_by_key(|cga| {
            let r = pixel.r().abs_diff(cga.r()) as u32;
            let g = pixel.g().abs_diff(cga.g()) as u32;
            let b = pixel.b().abs_diff(cga.b()) as u32;

            r * r + g * g + b * b
        })
        .unwrap()
}

fn distance(pixel: Vector3<i32>, cga: Vector3<i32>) -> i32 {
    let r = pixel[0] - cga[0];
    let g = pixel[1] - cga[1];
    let b = pixel[2] - cga[2];

    r * r + g * g + b * b
}
