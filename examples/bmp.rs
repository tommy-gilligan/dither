use embedded_graphics::{
    primitives::Rectangle,
    pixelcolor::Rgb888,
    prelude::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder,
    SimulatorDisplay,
    Window,
};
use tinybmp::Bmp;
use nalgebra::Vector3;

use dither::Dither;

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let mut display = SimulatorDisplay::<Rgb888>::new(bmp.size());

    display
        .fill_contiguous(
            &Rectangle::new(Point::zero(), bmp.size()),
            Dither::<_, _, 256, 257>::new(
                bmp.pixels().map(|c| {
                    let color = c.1;
                    Vector3::<i16>::new(
                        color.r().into(),
                        color.g().into(),
                        color.b().into()
                    )
                }),
                closest
            )
        )
        .unwrap();

    Window::new("Mona Lisa", &OutputSettingsBuilder::new().build()).show_static(&display);

    Ok(())
}

pub fn closest(pixel: Rgb888) -> Rgb888 {
    [
        Rgb888::new(0x00, 0x00, 0x00),
        Rgb888::new(0x55, 0x55, 0x55),
        Rgb888::new(0xAA, 0xAA, 0xAA),
        Rgb888::new(0xff, 0xff, 0xff),
        // maroon
        Rgb888::new(0xAA, 0x00, 0x00),
        // purple
        Rgb888::new(0xAA, 0x00, 0xAA),
        // brown
        Rgb888::new(0xAA, 0x55, 0x00),
        // tomato
        Rgb888::new(0xFF, 0x55, 0x55),
    ].into_iter()
        .min_by_key(|cga| {
            <u16 as Into<u32>>::into(<u8 as Into<u16>>::into(pixel.r().abs_diff(cga.r())).pow(2)) +
            <u16 as Into<u32>>::into(<u8 as Into<u16>>::into(pixel.g().abs_diff(cga.g())).pow(2)) +
            <u16 as Into<u32>>::into(<u8 as Into<u16>>::into(pixel.b().abs_diff(cga.b())).pow(2))
        })
        .unwrap()
}
