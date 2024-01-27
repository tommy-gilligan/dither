#![no_std]

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::{Rgb888, RgbColor},
    primitives::Rectangle,
    Pixel,
};

use nalgebra::Vector3;

#[cfg(feature = "cga")]
pub mod cga;
#[cfg(feature = "terminal")]
pub mod terminal;
mod wrapping_vec;

pub struct DitherTarget<'a, Display, F, const WIDTH_PLUS_ONE: usize>
where
    F: Fn(Rgb888) -> Display::Color,
    Display: DrawTarget + OriginDimensions,
    Rgb888: From<<Display as DrawTarget>::Color>,
{
    display: &'a mut Display,
    closest_color_fn: F,
}

impl<'a, Display, F, const WIDTH_PLUS_ONE: usize> DitherTarget<'a, Display, F, WIDTH_PLUS_ONE>
where
    F: Fn(Rgb888) -> Display::Color,
    Display: DrawTarget + OriginDimensions,
    Rgb888: From<<Display as DrawTarget>::Color>,
{
    pub fn new(display: &'a mut Display, closest_color_fn: F) -> Self {
        Self {
            display,
            closest_color_fn,
        }
    }
}

fn vector_from_rgb<C>(rgb: C) -> Vector3<i16>
where
    C: RgbColor,
{
    Vector3::<i16>::new(rgb.r().into(), rgb.g().into(), rgb.b().into())
}

fn rgb_from_vector(vector: Vector3<i16>) -> Rgb888 {
    Rgb888::new(
        vector[0].clamp(0, 255).try_into().unwrap_or(0x00),
        vector[1].clamp(0, 255).try_into().unwrap_or(0x00),
        vector[2].clamp(0, 255).try_into().unwrap_or(0x00),
    )
}

impl<'a, Display, F, const WIDTH_PLUS_ONE: usize> DrawTarget
    for DitherTarget<'a, Display, F, WIDTH_PLUS_ONE>
where
    F: Fn(Rgb888) -> Display::Color,
    Display: DrawTarget + OriginDimensions,
    Rgb888: From<<Display as DrawTarget>::Color>,
{
    type Color = Rgb888;
    type Error = Display::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let mut vectors = pixels
            .into_iter()
            .map(|pixel| vector_from_rgb::<Rgb888>(pixel.1));
        let mut buffer: crate::wrapping_vec::WrappingVec<Vector3<i16>, WIDTH_PLUS_ONE> =
            crate::wrapping_vec::WrappingVec::new(&mut vectors);

        self.display.fill_contiguous(
            &Rectangle::new(Point::zero(), self.size()),
            vectors.map(|vector| {
                let closest_color: Display::Color =
                    (self.closest_color_fn)(rgb_from_vector(buffer[0]));
                let closest_color_rgb: Rgb888 = closest_color.into();
                let quant_error: Vector3<i16> = buffer[0] - vector_from_rgb(closest_color_rgb);

                buffer[1] += (quant_error * 7) / 16;
                buffer[WIDTH_PLUS_ONE - 2] += (quant_error * 3) / 16;
                buffer[WIDTH_PLUS_ONE - 1] += (quant_error * 5) / 16;
                buffer[WIDTH_PLUS_ONE] += quant_error / 16;

                buffer.push(vector);

                closest_color
            }),
        )
    }
}

impl<'a, Display, F, const WIDTH_PLUS_ONE: usize> OriginDimensions
    for DitherTarget<'a, Display, F, WIDTH_PLUS_ONE>
where
    F: Fn(Rgb888) -> Display::Color,
    Display: DrawTarget + OriginDimensions,
    Rgb888: From<<Display as DrawTarget>::Color>,
{
    fn size(&self) -> Size {
        self.display.size()
    }
}
