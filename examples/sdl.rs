use cga::CGAColor;
use dither::embedded_graphics::DitherTarget;
use dither::vector::Vector;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::Bmp;

use fixed::types::extra::U16;
type Num = fixed::FixedI32<U16>;

const WIDTH: usize = 256;
const HEIGHT: usize = 383;

pub fn rgb_distance(a: Vector<Num>, c: Rgb888) -> fixed::FixedU32<U16> {
    a.0.abs_diff(Num::from_num(c.r()))
        + a.1.abs_diff(Num::from_num(c.g()))
        + a.2.abs_diff(Num::from_num(c.b()))
}

pub fn cga_to_rgb(x: cga::CGAColor) -> Rgb888 {
    cga::RGB_DISPLAY_PAIRS
        .iter()
        .find(|(cga, _)| *cga == x)
        .unwrap()
        .1
}

fn rgb_to_cga(source: Vector<Num>) -> (CGAColor, Vector<Num>) {
    let pair = cga::RGB_DISPLAY_PAIRS
        .iter()
        .min_by_key(|(_, rgb)| rgb_distance(source, *rgb))
        .unwrap();

    (pair.0, source - Vector::from_rgb888(pair.1))
}

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let size = Size::new(WIDTH as u32, HEIGHT as u32);

    let mut simulator_display = SimulatorDisplay::<Rgb888>::new(size);
    let mut cga: cga::FakeCGA<SimulatorDisplay<Rgb888>, _> =
        cga::FakeCGA::new(&mut simulator_display, &cga_to_rgb);

    let mut display: DitherTarget<'_, cga::FakeCGA<SimulatorDisplay<Rgb888>, _>, _, WIDTH> =
        DitherTarget::new(&mut cga, &rgb_to_cga);

    bmp.draw(&mut display).unwrap();

    Window::new("Mona Lisa", &OutputSettingsBuilder::new().build()).show_static(&simulator_display);
    Ok(())
}
