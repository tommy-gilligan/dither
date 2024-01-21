#![no_std]

use embedded_graphics_core::{
    Pixel,
    pixelcolor::{Rgb888, RgbColor},
    primitives::Rectangle,
    draw_target::DrawTarget,
    geometry::{Point, OriginDimensions, Size},
};

use nalgebra::Vector3;
mod wrapping_vec;
#[cfg(std)]
pub mod cga;
#[cfg(std)]
pub mod terminal;

pub struct DitherTarget<'a, Display, const WIDTH: usize, const WIDTH_PLUS_ONE: usize>
where
Display: DrawTarget + OriginDimensions,
Display::Color: From<Rgb888>,
Rgb888: From<<Display as DrawTarget>::Color>
{
    display: &'a mut Display,
}


impl <'a, Display, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> DitherTarget<'a, Display, WIDTH, WIDTH_PLUS_ONE>
where
Display: DrawTarget + OriginDimensions,
Display::Color: From<Rgb888>,
Rgb888: From<<Display as DrawTarget>::Color>
{
    pub fn new(display: &'a mut Display) -> Self {
        Self {
            display,
        }
    }
}

fn vector_from_rgb(rgb: Rgb888) -> Vector3<i16> {
    Vector3::<i16>::new(
       rgb.r().into(),
       rgb.g().into(),
       rgb.b().into()
    )
}

fn rgb_from_vector(vector: Vector3<i16>) -> Rgb888 {
    Rgb888::new(
        vector[0].clamp(0, 255).try_into().unwrap_or(0x00),
        vector[1].clamp(0, 255).try_into().unwrap_or(0x00),
        vector[2].clamp(0, 255).try_into().unwrap_or(0x00),
    )
}

impl <'a, Display, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> DrawTarget for DitherTarget<'a, Display, WIDTH, WIDTH_PLUS_ONE>
where
Display: DrawTarget + OriginDimensions,
Display::Color: From<Rgb888>,
Rgb888: From<<Display as DrawTarget>::Color>
{
    type Color = Rgb888;
    type Error = Display::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
       where I: IntoIterator<Item = Pixel<Self::Color>>
   {
         let mut vectors = pixels.into_iter().map(|pixel| vector_from_rgb(pixel.1));
         let mut buffer: crate::wrapping_vec::WrappingVec<Vector3<i16>, WIDTH_PLUS_ONE> = crate::wrapping_vec::WrappingVec::new(&mut vectors);

         self.display.fill_contiguous(
             &Rectangle::new(Point::zero(), self.size()),
             vectors.map(|vector| {
                let closest_color: Display::Color = rgb_from_vector(buffer[0]).into();
                let quant_error = buffer[0] - vector_from_rgb(closest_color.into());

                buffer[1] += (quant_error * 7) / 16;
                buffer[WIDTH - 1] += (quant_error * 3) / 16;
                buffer[WIDTH] += (quant_error * 5) / 16;
                buffer[WIDTH + 1] += quant_error / 16;

                buffer.push(vector);

                closest_color.into()
             })
         )
   }
}

impl <'a, Display, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> OriginDimensions for DitherTarget<'a, Display, WIDTH, WIDTH_PLUS_ONE>
where
Display: DrawTarget + OriginDimensions,
Display::Color: From<Rgb888>,
Rgb888: From<<Display as DrawTarget>::Color>
{
    fn size(&self) -> Size {
        self.display.size()
    }
}
