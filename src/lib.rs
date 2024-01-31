#![no_std]

mod accumulator;
#[cfg(feature = "cga")]
pub mod cga;
pub mod color_cube;
#[cfg(feature = "terminal")]
pub mod terminal;
mod wrapping_vec;

use accumulator::Accumulator;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::Rgb888,
    primitives::Rectangle,
    Pixel,
};

pub struct DitherTarget<'a, Display, F, const WIDTH_PLUS_ONE: usize>
where
    F: Fn(Rgb888) -> (Display::Color, (i16, i16, i16)),
    Display: DrawTarget + OriginDimensions,
{
    display: &'a mut Display,
    closest_color_fn: &'a F,
    accumulation_buffer: crate::wrapping_vec::WrappingVec<Accumulator, WIDTH_PLUS_ONE>,
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
            accumulation_buffer: crate::wrapping_vec::WrappingVec::new(&mut core::iter::repeat(
                Accumulator::default(),
            )),
        }
    }

    fn initialize_accumulation_buffer<I>(&mut self, pixels: &mut I)
    where
        I: Iterator<Item = Accumulator>,
    {
        self.accumulation_buffer = crate::wrapping_vec::WrappingVec::new(pixels);
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
        let mut pixels = pixels.into_iter().map(|pixel| pixel.1.into());

        self.initialize_accumulation_buffer(&mut pixels);

        self.display.fill_contiguous(
            &Rectangle::new(Point::zero(), self.size()),
            pixels.map(|finished_accumulator| {
                let (closest_color, quantization_error): (Display::Color, (i16, i16, i16)) =
                    (self.closest_color_fn)(self.accumulation_buffer[0].into());

                let quantization_error = Accumulator::new(quantization_error);

                // assert!(
                //     (self.closest_color_fn)(closest_color_rgb) == closest_color
                // );

                self.accumulation_buffer[1] += (quantization_error * 7) >> 4;
                self.accumulation_buffer[WIDTH_PLUS_ONE - 2] += (quantization_error * 3) >> 4;
                self.accumulation_buffer[WIDTH_PLUS_ONE - 1] += (quantization_error * 5) >> 4;
                self.accumulation_buffer[WIDTH_PLUS_ONE] += (quantization_error) >> 4;

                self.accumulation_buffer.push(finished_accumulator);

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
