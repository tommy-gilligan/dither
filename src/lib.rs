use embedded_graphics_core::{
    geometry::Size,
    pixelcolor::{Rgb888, RgbColor},
};
use rand::prelude::*;

use nalgebra::Vector3;

pub fn color_to_vector<C>(color: C) -> Vector3<i32>
where
    C: RgbColor,
{
    Vector3::<i32>::new(color.r() as i32, color.g() as i32, color.b() as i32)
}

pub fn vector_to_colora(color: Vector3<i32>) -> Rgb888 {
    Rgb888::new(
        color[0].clamp(0, 255).try_into().unwrap_or(0x00),
        color[1].clamp(0, 255).try_into().unwrap_or(0x00),
        color[2].clamp(0, 255).try_into().unwrap_or(0x00),
    )
}

pub fn dither<F, I>(pixels: I, size: Size, f: F) -> impl Iterator<Item = Rgb888>
where
    F: Fn(Vector3<i32>) -> Vector3<i32>,
    I: Iterator<Item = Rgb888>,
{
    let pixels_copy_a: Vec<Vector3<i32>> = pixels.map(color_to_vector).collect();
    let mut pixels_copy: Vec<Vec<Vector3<i32>>> = pixels_copy_a
        .chunks(size.width.try_into().unwrap())
        .map(|a| a.to_vec())
        .collect();
    let mut peekable = pixels_copy.iter_mut().enumerate().peekable();

    while let Some((i, row)) = peekable.next() {
        let row_next = peekable.peek_mut();

        for x in 0..(size.width as usize) {
            let x = if i % 2 == 0 {
                x
            } else {
                size.width as usize - 1 - x
            };
            let oldpixel = row[x];
            let newpixel = f(oldpixel);
            row[x] = newpixel;
            let quant_error = oldpixel - newpixel;

            if (x + 1) < (size.width as usize) {
                row[x + 1] += quant_error * 7 / 16
            }
            if (x as isize - 1) >= 0 {
                if let Some((_, row_next)) = row_next {
                    row_next[x - 1] += quant_error * 3 / 16
                }
            }
            if let Some((_, row_next)) = row_next {
                row_next[x] += quant_error * 5 / 16
            }
            if (x + 1) < (size.width as usize) {
                if let Some((_, row_next)) = row_next {
                    row_next[x + 1] += quant_error * 1 / 16
                }
            }
        }
    }

    pixels_copy
        .into_iter()
        .flat_map(|row| row.into_iter().map(vector_to_colora))
}

#[test]
fn test_dither() {
    let mut image: Vec<Rgb888> = vec![
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
    let output: Vec<_> = dither(image.into_iter(), Size::new(3, 3), closest).collect();

    assert_eq!(
        output,
        vec![
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

const CGA_PALETTE: [Vector3<i32>; 14] = [
    Vector3::<i32>::new(0x00, 0x00, 0x00),
    Vector3::<i32>::new(0x00, 0x00, 0xAA),
    Vector3::<i32>::new(0x00, 0xAA, 0x00),
    Vector3::<i32>::new(0x00, 0xAA, 0xAA),
    Vector3::<i32>::new(0xAA, 0x00, 0x00),
    Vector3::<i32>::new(0xAA, 0x00, 0xAA),
    Vector3::<i32>::new(0xAA, 0xAA, 0xAA),
    Vector3::<i32>::new(0xAA, 0x55, 0x00),
    Vector3::<i32>::new(0xff, 0xff, 0xff),
    Vector3::<i32>::new(0x55, 0x55, 0x55),
    Vector3::<i32>::new(0x55, 0x55, 0xFF),
    Vector3::<i32>::new(0xFF, 0xFF, 0x55),
    Vector3::<i32>::new(0xFF, 0x55, 0x55),
    Vector3::<i32>::new(0x55, 0xFF, 0x55),
];

pub fn closest(pixel: Vector3<i32>) -> Vector3<i32> {
    CGA_PALETTE
        .into_iter()
        .min_by_key(|cga| {
            let r = pixel[0] - cga[0];
            let g = pixel[1] - cga[1];
            let b = pixel[2] - cga[2];

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

const THRESHOLD: i32 = 1000;

pub fn stochastic_closest(pixel: Vector3<i32>) -> Vector3<i32> {
    let mut palette = CGA_PALETTE.to_vec();
    palette.sort_by_key(|cga| distance(pixel, *cga));
    let mut filtered: Vec<Vector3<i32>> = palette.clone().into_iter().filter(|cga| distance(pixel, *cga) < THRESHOLD).collect();
    let mut rng = rand::thread_rng();
    filtered.shuffle(&mut rng);
    if filtered.len() > 0 {
        filtered[0]
    } else {
        palette[0]
    }
}
