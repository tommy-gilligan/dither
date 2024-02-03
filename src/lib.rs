#![no_std]

mod accumulator;
mod wrapping_vec;

#[cfg(feature = "cga")]
pub mod cga;
#[cfg(feature = "color_cube")]
pub mod color_cube;
#[cfg(feature = "terminal")]
pub mod terminal;
mod dither;

use accumulator::Accumulator;
pub type QuantizationError = Accumulator;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::{PixelColor, RgbColor},
    primitives::Rectangle,
    Pixel,
};

use core::marker::PhantomData;

pub struct DitherTarget<'a, Display, C, F, const WIDTH: usize>
where
    F: Fn(C) -> (Display::Color, QuantizationError),
    Display: DrawTarget + OriginDimensions,
    C: PixelColor + From<Accumulator> + RgbColor,
{
    display: &'a mut Display,
    closest_color_fn: &'a F,
    accumulation_buffer: crate::wrapping_vec::WrappingVec<Accumulator, WIDTH>,
    phantom: PhantomData<C>,
}

impl<'a, Display, C, F, const WIDTH: usize> DitherTarget<'a, Display, C, F, WIDTH>
where
    F: Fn(C) -> (Display::Color, QuantizationError),
    Display: DrawTarget + OriginDimensions,
    C: PixelColor + From<Accumulator> + Into<Accumulator> + RgbColor,
{
    pub fn new(display: &'a mut Display, closest_color_fn: &'a F) -> Self {
        Self {
            display,
            closest_color_fn,
            accumulation_buffer: crate::wrapping_vec::WrappingVec::new(&mut core::iter::repeat(
                Accumulator::default(),
            )),
            phantom: PhantomData,
        }
    }

    fn initialize_accumulation_buffer<I>(&mut self, pixels: &mut I)
    where
        I: Iterator<Item = Accumulator>,
    {
        self.accumulation_buffer = crate::wrapping_vec::WrappingVec::new(pixels);
    }
}

impl<'a, Display, C, F, const WIDTH: usize> DrawTarget for DitherTarget<'a, Display, C, F, WIDTH>
where
    F: Fn(C) -> (Display::Color, QuantizationError),
    Display: DrawTarget + OriginDimensions,
    C: PixelColor + From<Accumulator> + RgbColor,
{
    type Color = C;
    type Error = Display::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let mut pixels = pixels.into_iter().map(|pixel| pixel.1.into());

        self.initialize_accumulation_buffer(&mut pixels);

        self.display.fill_contiguous(
            &Rectangle::new(Point::zero(), self.size()),
            pixels.map(|horizon_pixel| {
                let (dithered_color, quantization_error): (Display::Color, QuantizationError) =
                    (self.closest_color_fn)(self.accumulation_buffer[0].into());

                // assert!(
                //     (self.closest_color_fn)(closest_color_rgb) == dithered_color
                // );

                self.accumulation_buffer[1] += (quantization_error * 7) >> 4;
                self.accumulation_buffer[WIDTH - 1] += (quantization_error * 3) >> 4;
                self.accumulation_buffer[WIDTH] += (quantization_error * 5) >> 4;
                self.accumulation_buffer[WIDTH + 1] += (quantization_error) >> 4;

                self.accumulation_buffer.push(horizon_pixel);

                dithered_color
            }),
        )
    }
}

impl<'a, Display, C, F, const WIDTH: usize> OriginDimensions
    for DitherTarget<'a, Display, C, F, WIDTH>
where
    F: Fn(C) -> (Display::Color, QuantizationError),
    Display: DrawTarget + OriginDimensions,
    C: PixelColor + From<Accumulator> + RgbColor,
{
    fn size(&self) -> Size {
        self.display.size()
    }
}
