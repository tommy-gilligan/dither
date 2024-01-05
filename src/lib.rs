use core::convert::TryFrom;
use embedded_graphics_core::{
    geometry::Size,
    pixelcolor::{Rgb888, RgbColor},
};

use nalgebra::Vector3;

pub fn color_to_vector<C>(color: C) -> Vector3<i32>
where
    C: RgbColor,
{
    Vector3::<i32>::new(color.r() as i32, color.g() as i32, color.b() as i32)
}

pub fn vector_to_color(color: Vector3<i32>) -> Result<[u8; 3], <u8 as TryFrom<i32>>::Error> {
    Ok([
        color[0].try_into()?,
        color[1].try_into()?,
        color[2].try_into()?,
    ])
}

pub fn vector_to_colora(color: Vector3<i32>) -> Rgb888 {
    Rgb888::new(
        color[0].try_into().unwrap_or(0x00),
        color[1].try_into().unwrap_or(0x00),
        color[2].try_into().unwrap_or(0x00),
    )
}

pub fn dither<F, I>(pixels: I, size: Size, f: F) -> impl Iterator<Item = Rgb888>
where
    F: Fn(Rgb888) -> Rgb888,
    I: Iterator<Item = Rgb888>,
{
    let mut pixels_copy_a: Vec<Vector3<i32>> = pixels.map(color_to_vector).collect();
    let mut pixels_copy: Vec<Vec<Vector3<i32>>> = pixels_copy_a.chunks(size.width.try_into().unwrap()).map(|a| a.to_vec()).collect();
    let mut peekable = pixels_copy.iter_mut().peekable();

    while let Some(row) = peekable.next() {
        let mut row_next = peekable.peek_mut();

        for x in 0..(size.width as usize) {
            let oldpixel = row[x];
            let newpixel = color_to_vector(f(vector_to_colora(oldpixel)));
            row[x] = newpixel;
            let quant_error = oldpixel - newpixel;

            if (x + 1) < (size.width as usize) {
                row[x + 1] += quant_error * 7 / 16
            }
            if (x as isize - 1) >= 0 {
                if row_next.is_some() {
                    row_next.as_mut().unwrap()[x - 1] += quant_error * 3 / 16
                }
            }
            if row_next.is_some() {
                row_next.as_mut().unwrap()[x] += quant_error * 5 / 16;
            }

            if (x + 1) < (size.width as usize) {
                if row_next.is_some() {
                    row_next.as_mut().unwrap()[x + 1] += quant_error * 1 / 16
                }
            }
        }
    }

    pixels_copy.into_iter().flat_map(|row| row.into_iter().map(vector_to_colora))
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

const CGA_PALETTE: [Rgb888; 13] = [
    Rgb888::new(0x00, 0x00, 0x00),
    Rgb888::new(0x00, 0x00, 0xAA),
    Rgb888::new(0x00, 0xAA, 0x00),
    Rgb888::new(0x00, 0xAA, 0xAA),
    Rgb888::new(0xAA, 0x00, 0x00),
    Rgb888::new(0xAA, 0x00, 0xAA),
    Rgb888::new(0xAA, 0xAA, 0xAA),
    Rgb888::new(0xAA, 0x55, 0x00),
    Rgb888::new(0xFF, 0xFF, 0x55),
    Rgb888::new(0xFF, 0x55, 0x55),
    Rgb888::new(0x55, 0xFF, 0x55),
    Rgb888::new(0x55, 0x55, 0xFF),
    Rgb888::new(0x55, 0x55, 0x55),
];

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
