use dither::{cga, terminal::SimulatorDisplay, DitherTarget};
use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*};
use tinybmp::Bmp;

const WIDTH: usize = 256;
const HEIGHT: usize = 383;

pub fn rgb_distance<C, D>(a: C, c: D) -> u16
where
    C: embedded_graphics_core::pixelcolor::RgbColor,
    D: embedded_graphics_core::pixelcolor::RgbColor,
{
    (a.r() as u16).abs_diff(c.r() as u16)
        + (a.g() as u16).abs_diff(c.g() as u16)
        + (a.b() as u16).abs_diff(c.b() as u16)
}

pub fn rgb_to_cga(x: Rgb888) -> cga::CGAColor {
    cga::RGB_DISPLAY_PAIRS
        .iter()
        .min_by_key(|(_, rgb)| rgb_distance(x, *rgb))
        .unwrap()
        .0
}

pub fn cga_to_rgb(x: cga::CGAColor) -> Rgb888 {
    cga::RGB_DISPLAY_PAIRS
        .iter()
        .find(|(cga, _)| *cga == x)
        .unwrap()
        .1
}

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let size = Size::new(WIDTH as u32, HEIGHT as u32);

    let mut simulator_display = SimulatorDisplay::new(size);
    let mut cga: cga::FakeCGA<SimulatorDisplay, _> =
        cga::FakeCGA::new(&mut simulator_display, &cga_to_rgb);
    let mut display: DitherTarget<'_, cga::FakeCGA<SimulatorDisplay, _>, _, _, { WIDTH + 1 }> =
        DitherTarget::new(&mut cga, &rgb_to_cga, &cga_to_rgb);
    bmp.draw(&mut display).unwrap();

    Ok(())
}
