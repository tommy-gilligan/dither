use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    pixelcolor::{raw::RawU32, Rgb888},
    prelude::*,
};

pub struct FakeCGA<'a, Display, F>
where
    Display: DrawTarget<Color = Rgb888> + OriginDimensions,
    F: Fn(CGAColor) -> Rgb888,
{
    display: &'a mut Display,
    color_conversion_fn: &'a F,
}

impl<'a, Display, F> FakeCGA<'a, Display, F>
where
    Display: DrawTarget<Color = Rgb888> + OriginDimensions,
    F: Fn(CGAColor) -> Rgb888,
{
    pub fn new(display: &'a mut Display, color_conversion_fn: &'a F) -> Self {
        Self {
            display,
            color_conversion_fn,
        }
    }
}

impl<'a, Display, F> OriginDimensions for FakeCGA<'a, Display, F>
where
    Display: DrawTarget<Color = Rgb888> + OriginDimensions,
    F: Fn(CGAColor) -> Rgb888,
{
    fn size(&self) -> Size {
        self.display.size()
    }
}

impl<'a, Display, F> DrawTarget for FakeCGA<'a, Display, F>
where
    Display: DrawTarget<Color = Rgb888> + OriginDimensions,
    F: Fn(CGAColor) -> Rgb888,
{
    type Color = CGAColor;
    type Error = <Display as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.display.draw_iter(
            pixels
                .into_iter()
                .map(|pixel| Pixel(pixel.0, (self.color_conversion_fn)(pixel.1))),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CGAColor {
    #[default]
    Black,
    DarkGray,
    Blue,
    LightBlue,
    Green,
    LightGreen,
    Cyan,
    LightCyan,
    Red,
    LightRed,
    Magenta,
    LightMagenta,
    Brown,
    Yellow,
    LightGray,
    White,
}

pub const RGB_DISPLAY_PAIRS: [(CGAColor, Rgb888); 16] = [
    (CGAColor::Black, Rgb888::new(0x00, 0x00, 0x00)),
    (CGAColor::DarkGray, Rgb888::new(0x55, 0x55, 0x55)),
    (CGAColor::Blue, Rgb888::new(0x00, 0x00, 0xaa)),
    (CGAColor::LightBlue, Rgb888::new(0x55, 0x55, 0xff)),
    (CGAColor::Green, Rgb888::new(0x00, 0xaa, 0x00)),
    (CGAColor::LightGreen, Rgb888::new(0x55, 0xff, 0x55)),
    (CGAColor::Cyan, Rgb888::new(0x00, 0xaa, 0xaa)),
    (CGAColor::LightCyan, Rgb888::new(0x55, 0xff, 0xff)),
    (CGAColor::Red, Rgb888::new(0xaa, 0x00, 0x00)),
    (CGAColor::LightRed, Rgb888::new(0xff, 0x55, 0x55)),
    (CGAColor::Magenta, Rgb888::new(0xaa, 0x00, 0xaa)),
    (CGAColor::LightMagenta, Rgb888::new(0xff, 0x55, 0xff)),
    (CGAColor::Brown, Rgb888::new(0xaa, 0x55, 0x00)),
    (CGAColor::Yellow, Rgb888::new(0xff, 0xff, 0x55)),
    (CGAColor::LightGray, Rgb888::new(0xaa, 0xaa, 0xaa)),
    (CGAColor::White, Rgb888::new(0xff, 0xff, 0xff)),
];

impl PixelColor for CGAColor {
    type Raw = RawU32;
}
