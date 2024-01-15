use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder,
    SimulatorDisplay,
    Window,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    pixelcolor::{PixelColor, raw::RawU32}
};
use tinybmp::Bmp;
use dither::DitherTarget;

const WIDTH: usize = 256;
const HEIGHT: usize = 383;

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let size = Size::new(WIDTH as u32, HEIGHT as u32);

    let mut simulator_display = SimulatorDisplay::<Rgb888>::new(size);
    let mut cga: FakeCGA = FakeCGA::new(&mut simulator_display);
    let mut display: DitherTarget<'_, _, 256, 257> = DitherTarget::new(&mut cga);
    bmp.draw(&mut display).unwrap();

    Window::new(
        "Mona Lisa",
        &OutputSettingsBuilder::new().build()
    ).show_static(&simulator_display);
    Ok(())
}

struct FakeCGA<'a> {
    display: &'a mut SimulatorDisplay::<Rgb888>
}

impl <'a>FakeCGA<'a> {
    fn new(display: &'a mut SimulatorDisplay::<Rgb888>) -> Self {
        Self { display }
    }
}

impl <'a>OriginDimensions for FakeCGA<'a> {
    fn size(&self) -> Size {
        self.display.size()
    }
}

impl <'a>DrawTarget for FakeCGA<'a> {
    type Color = CGAColor;
    type Error = <SimulatorDisplay::<Rgb888> as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
       where I: IntoIterator<Item = Pixel<Self::Color>> {
           self.display.draw_iter(pixels.into_iter().map(|pixel| {
               Pixel(pixel.0, pixel.1.into())
           }))
   }
}

#[derive(Copy, Clone, PartialEq)]
enum CGAColor {
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
