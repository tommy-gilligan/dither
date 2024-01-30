#![no_std]

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::{Rgb888, RgbColor},
    primitives::Rectangle,
    Pixel,
};

#[cfg(feature = "cga")]
pub mod cga;
pub mod color_cube;
#[cfg(feature = "terminal")]
pub mod terminal;
mod wrapping_vec;

pub struct DitherTarget<'a, Display, F, const WIDTH_PLUS_ONE: usize>
where
    F: Fn(Rgb888) -> (Display::Color, (i16, i16, i16)),
    Display: DrawTarget + OriginDimensions,
{
    display: &'a mut Display,
    closest_color_fn: &'a F,
}

impl<'a, Display, F, const WIDTH_PLUS_ONE: usize> DitherTarget<'a, Display, F, WIDTH_PLUS_ONE>
where
    F: Fn(Rgb888) -> (Display::Color, (i16, i16, i16)),
    Display: DrawTarget + OriginDimensions,
{
    pub fn new(display: &'a mut Display, closest_color_fn: &'a F) -> Self {
        Self {
            display,
            closest_color_fn,
        }
    }
}

impl<'a, Display, F, const WIDTH_PLUS_ONE: usize> DrawTarget
    for DitherTarget<'a, Display, F, WIDTH_PLUS_ONE>
where
    F: Fn(Rgb888) -> (Display::Color, (i16, i16, i16)),
    Display: DrawTarget + OriginDimensions,
{
    type Color = Rgb888;
    type Error = Display::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let mut vectors = pixels
            .into_iter()
            .map(|pixel| (pixel.1.r().into(), pixel.1.g().into(), pixel.1.b().into()));
        let mut buffer: crate::wrapping_vec::WrappingVec<(i16, i16, i16), WIDTH_PLUS_ONE> =
            crate::wrapping_vec::WrappingVec::new(&mut vectors);

        self.display.fill_contiguous(
            &Rectangle::new(Point::zero(), self.size()),
            vectors.map(|vector| {
                let (closest_color, quant_error): (Display::Color, (i16, i16, i16)) =
                    (self.closest_color_fn)(Rgb888::new(
                        buffer[0].0.clamp(0, 255).try_into().unwrap_or(0x00),
                        buffer[0].1.clamp(0, 255).try_into().unwrap_or(0x00),
                        buffer[0].2.clamp(0, 255).try_into().unwrap_or(0x00),
                    ));

                // assert!((self.closest_color_fn)(closest_color_rgb) == closest_color);

                buffer[1] = (
                    buffer[1].0 + (quant_error.0 * 7) / 16,
                    buffer[1].1 + (quant_error.1 * 7) / 16,
                    buffer[1].2 + (quant_error.2 * 7) / 16,
                );
                buffer[WIDTH_PLUS_ONE - 2] = (
                    buffer[WIDTH_PLUS_ONE - 2].0 + (quant_error.0 * 3) / 16,
                    buffer[WIDTH_PLUS_ONE - 2].1 + (quant_error.1 * 3) / 16,
                    buffer[WIDTH_PLUS_ONE - 2].2 + (quant_error.2 * 3) / 16,
                );
                buffer[WIDTH_PLUS_ONE - 1] = (
                    buffer[WIDTH_PLUS_ONE - 1].0 + (quant_error.0 * 5) / 16,
                    buffer[WIDTH_PLUS_ONE - 1].1 + (quant_error.1 * 5) / 16,
                    buffer[WIDTH_PLUS_ONE - 1].2 + (quant_error.2 * 5) / 16,
                );
                buffer[WIDTH_PLUS_ONE] = (
                    buffer[WIDTH_PLUS_ONE].0 + (quant_error.0) / 16,
                    buffer[WIDTH_PLUS_ONE].1 + (quant_error.1) / 16,
                    buffer[WIDTH_PLUS_ONE].2 + (quant_error.2) / 16,
                );

                buffer.push(vector);

                closest_color
            }),
        )
    }
}

impl<'a, Display, F, const WIDTH_PLUS_ONE: usize> OriginDimensions
    for DitherTarget<'a, Display, F, WIDTH_PLUS_ONE>
where
    F: Fn(Rgb888) -> (Display::Color, (i16, i16, i16)),
    Display: DrawTarget + OriginDimensions,
{
    fn size(&self) -> Size {
        self.display.size()
    }
}
