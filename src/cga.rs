use embedded_graphics_core::{
    pixelcolor::{Rgb888, raw::RawU32},
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    prelude::*,
};

pub struct FakeCGA<'a, Display> where Display: DrawTarget<Color = Rgb888> + OriginDimensions {
    display: &'a mut Display
}

impl <'a, Display>FakeCGA<'a, Display> where Display: DrawTarget<Color = Rgb888> + OriginDimensions {
    pub fn new(display: &'a mut Display) -> Self {
        Self { display }
    }
}

impl <'a, Display>OriginDimensions for FakeCGA<'a, Display> where Display: DrawTarget<Color = Rgb888> + OriginDimensions {
    fn size(&self) -> Size {
        self.display.size()
    }
}

 
impl <'a, Display>DrawTarget for FakeCGA<'a, Display> where Display: DrawTarget<Color = Rgb888> + OriginDimensions {
    type Color = CGAColor;
    type Error = <Display as DrawTarget>::Error;
// <Display as DrawTarget>::Color: From<CGAColor>

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
       where I: IntoIterator<Item = Pixel<Self::Color>> {
           self.display.draw_iter(pixels.into_iter().map(|pixel| {
               Pixel(pixel.0, pixel.1.into())
           }))
   }
}

#[derive(Copy, Clone, PartialEq)]
pub enum CGAColor {
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
    White
}

const RGB_DISPLAY_PAIRS: [(CGAColor, Rgb888); 16] = [
    (CGAColor::Black,        Rgb888::new(0x00, 0x00, 0x00)),
    (CGAColor::DarkGray,     Rgb888::new(0x55, 0x55, 0x55)),
    (CGAColor::Blue,         Rgb888::new(0x00, 0x00, 0xaa)),
    (CGAColor::LightBlue,    Rgb888::new(0x55, 0x55, 0xff)),
    (CGAColor::Green,        Rgb888::new(0x00, 0xaa, 0x00)),
    (CGAColor::LightGreen,   Rgb888::new(0x55, 0xff, 0x55)),
    (CGAColor::Cyan,         Rgb888::new(0x00, 0xaa, 0xaa)),
    (CGAColor::LightCyan,    Rgb888::new(0x55, 0xff, 0xff)),
    (CGAColor::Red,          Rgb888::new(0xaa, 0x00, 0x00)),
    (CGAColor::LightRed,     Rgb888::new(0xff, 0x55, 0x55)),
    (CGAColor::Magenta,      Rgb888::new(0xaa, 0x00, 0xaa)),
    (CGAColor::LightMagenta, Rgb888::new(0xff, 0x55, 0xff)),
    (CGAColor::Brown,        Rgb888::new(0xaa, 0x55, 0x00)),
    (CGAColor::Yellow,       Rgb888::new(0xff, 0xff, 0x55)),
    (CGAColor::LightGray,    Rgb888::new(0xaa, 0xaa, 0xaa)),
    (CGAColor::White,        Rgb888::new(0xff, 0xff, 0xff)),
];

fn rgb_distance(a: Rgb888, c: Rgb888) -> u32 {
    let r: i32 = (a.r() as i32) - (c.r() as i32);
    let g: i32 = (a.g() as i32) - (c.g() as i32);
    let b: i32 = (a.b() as i32) - (c.b() as i32);

    ((r * r + g * g + b * b) as f64).sqrt() as u32
}

impl From<CGAColor> for Rgb888 {
    fn from(value: CGAColor) -> Self {
        RGB_DISPLAY_PAIRS.iter().find(|(cga, _)| *cga == value).unwrap().1
    }
}

impl From<Rgb888> for CGAColor {
    fn from(value: Rgb888) -> Self {
       RGB_DISPLAY_PAIRS
           .iter()
           .min_by_key(|(_, rgb)| rgb_distance(value, *rgb))
           .unwrap()
           .0
    }
}

impl PixelColor for CGAColor {
    type Raw = RawU32;
}
