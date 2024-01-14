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

pub struct DitherTarget<'a, Display, ClosestColor, const WIDTH: usize, const WIDTH_PLUS_ONE: usize>
where
Display: DrawTarget + OriginDimensions,
ClosestColor: Fn(Display::Color) -> Display::Color,
Display::Color: RgbColor + From<Rgb888>
{
    display: &'a mut Display,
    closest_color: ClosestColor,
}

impl <'a, Display, ClosestColor, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> DitherTarget<'a, Display, ClosestColor, WIDTH, WIDTH_PLUS_ONE>
where
Display: DrawTarget + OriginDimensions,
ClosestColor: Fn(Display::Color) -> Display::Color,
Display::Color: RgbColor + From<Rgb888>
{
    pub fn new(display: &'a mut Display, closest_color: ClosestColor) -> Self {
        Self {
            display,
            closest_color,
        }
    }
}

impl <'a, Display, ClosestColor, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> DrawTarget for DitherTarget<'a, Display, ClosestColor, WIDTH, WIDTH_PLUS_ONE>
where
Display: DrawTarget + OriginDimensions,
ClosestColor: Fn(Display::Color) -> Display::Color,
Display::Color: RgbColor + From<Rgb888>
{
    type Color = Display::Color;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
       where I: IntoIterator<Item = Pixel<Self::Color>>
   {
         let mut vectors = pixels.into_iter().map(|pixel| {
             let color = pixel.1;
             Vector3::<i16>::new(
                color.r().into(),
                color.g().into(),
                color.b().into()
             )
         });
         let mut buffer: crate::wrapping_vec::WrappingVec<Vector3<i16>, WIDTH_PLUS_ONE> = crate::wrapping_vec::WrappingVec::new(&mut vectors);

         let vx = vectors.map(|vector| {
            let lookup: Self::Color = Rgb888::new(
                buffer[0][0].clamp(0, 255).try_into().unwrap_or(0x00),
                buffer[0][1].clamp(0, 255).try_into().unwrap_or(0x00),
                buffer[0][2].clamp(0, 255).try_into().unwrap_or(0x00),
            ).into();
            let newpixel = (self.closest_color)(lookup);
            let quant_error = buffer[0] - Vector3::<i16>::new(
                newpixel.r().into(),
                newpixel.g().into(),
                newpixel.b().into()
            );
            buffer[1] += (quant_error * 7) / 16;
            buffer[WIDTH - 1] += (quant_error * 3) / 16;
            buffer[WIDTH] += (quant_error * 5) / 16;
            buffer[WIDTH + 1] += quant_error / 16;
            buffer.push(vector);
            newpixel
         });

         let _ = self.display.fill_contiguous(
             &Rectangle::new(Point::zero(), self.size()),
             vx
         );
         Ok(())
   }
}

impl <'a, Display, ClosestColor, const WIDTH: usize, const WIDTH_PLUS_ONE: usize> OriginDimensions for DitherTarget<'a, Display, ClosestColor, WIDTH, WIDTH_PLUS_ONE>
where
Display: DrawTarget + OriginDimensions,
ClosestColor: Fn(Display::Color) -> Display::Color,
Display::Color: RgbColor + From<Rgb888>
{
    fn size(&self) -> Size {
        self.display.size()
    }
}
