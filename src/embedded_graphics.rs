use crate::vector::Vector;
use crate::windowing_convolution::WindowingConvolution;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::Rgb888,
    primitives::Rectangle,
    Pixel,
};
use fixed::types::extra::U16;
type Num = fixed::FixedI32<U16>;

pub struct DitherTarget<'a, Display, F, const WIDTH: usize>
where
    Display: DrawTarget + OriginDimensions,
    F: Fn(Vector<Num>) -> (Display::Color, Vector<Num>),
{
    display: &'a mut Display,
    color_conversion_fn: &'a F,
}

impl<'a, Display, F, const WIDTH: usize> DitherTarget<'a, Display, F, WIDTH>
where
    Display: DrawTarget + OriginDimensions,
    F: Fn(Vector<Num>) -> (Display::Color, Vector<Num>),
{
    pub fn new(display: &'a mut Display, color_conversion_fn: &'a F) -> Self {
        Self {
            display,
            color_conversion_fn,
        }
    }
}

impl<'a, Display, F, const WIDTH: usize> DrawTarget for DitherTarget<'a, Display, F, WIDTH>
where
    Display: DrawTarget + OriginDimensions,
    F: Fn(Vector<Num>) -> (Display::Color, Vector<Num>),
{
    type Color = Rgb888;
    type Error = Display::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let pixels = pixels
            .into_iter()
            .map(|rgb888| Vector::<Num>::from_rgb888(rgb888.1));
        let kernel = [
            ((1, 0), Num::from_str("0.875").unwrap()),
            ((2, 0), Num::from_str("0.125").unwrap()),
        ];
        let convolution: WindowingConvolution<Vector<Num>, _, _, _, _, WIDTH> =
            WindowingConvolution::new(pixels, &kernel, self.color_conversion_fn);

        self.display
            .fill_contiguous(&Rectangle::new(Point::zero(), self.size()), convolution)
    }
}

impl<'a, Display, F, const WIDTH: usize> OriginDimensions for DitherTarget<'a, Display, F, WIDTH>
where
    Display: DrawTarget + OriginDimensions,
    F: Fn(Vector<Num>) -> (Display::Color, Vector<Num>),
{
    fn size(&self) -> Size {
        self.display.size()
    }
}
