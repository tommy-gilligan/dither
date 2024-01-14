use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder,
    SimulatorDisplay,
    Window,
};
use tinybmp::Bmp;
use dither::DitherTarget;

const WIDTH: usize = 256;
const HEIGHT: usize = 383;

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let size = Size::new(WIDTH as u32, HEIGHT as u32);

    let mut simulator_display = SimulatorDisplay::<Rgb888>::new(size);
    let mut display: DitherTarget<'_, _, _, 256, 257> = DitherTarget::new(&mut simulator_display, closest);
    bmp.draw(&mut display).unwrap();

    Window::new("Mona Lisa", &OutputSettingsBuilder::new().build()).show_static(&simulator_display);
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
