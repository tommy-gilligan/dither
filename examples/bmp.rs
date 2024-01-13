use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::Bmp;

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let mut display = SimulatorDisplay::<Rgb888>::new(bmp.size());

    display
        .fill_contiguous(
            &Rectangle::new(Point::zero(), bmp.size()),
            dither::Dither::new(bmp.size(), bmp.pixels().map(|c| c.1), dither::closest),
        )
        .unwrap();

    Window::new("Hello World", &OutputSettingsBuilder::new().build()).show_static(&display);

    Ok(())
}

const CGA_PALETTE: [Rgb888; 14] = [
    Rgb888::new(0x00, 0x00, 0x00),
    Rgb888::new(0x00, 0x00, 0xAA),
    Rgb888::new(0x00, 0xAA, 0x00),
    Rgb888::new(0x00, 0xAA, 0xAA),
    Rgb888::new(0xAA, 0x00, 0x00),
    Rgb888::new(0xAA, 0x00, 0xAA),
    Rgb888::new(0xAA, 0xAA, 0xAA),
    Rgb888::new(0xAA, 0x55, 0x00),
    Rgb888::new(0xff, 0xff, 0xff),
    Rgb888::new(0x55, 0x55, 0x55),
    Rgb888::new(0x55, 0x55, 0xFF),
    Rgb888::new(0xFF, 0xFF, 0x55),
    Rgb888::new(0xFF, 0x55, 0x55),
    Rgb888::new(0x55, 0xFF, 0x55),
];

pub fn closest(pixel: Rgb888) -> Rgb888 {
    CGA_PALETTE
        .into_iter()
        .min_by_key(|cga| {
            let r = pixel.r().abs_diff(cga.r()) as u32;
            let g = pixel.g().abs_diff(cga.g()) as u32;
            let b = pixel.b().abs_diff(cga.b()) as u32;

            r * r + g * g + b * b
        })
        .unwrap()
}
