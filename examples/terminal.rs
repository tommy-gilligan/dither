use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::*
};
use tinybmp::Bmp;
use dither::{DitherTarget, terminal::SimulatorDisplay, cga};

const WIDTH: usize = 256;
const HEIGHT: usize = 383;

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let size = Size::new(WIDTH as u32, HEIGHT as u32);

    let mut simulator_display = SimulatorDisplay::new(size);
    let mut cga: cga::FakeCGA<SimulatorDisplay> = cga::FakeCGA::new(&mut simulator_display);
    let mut display: DitherTarget<'_, _, { WIDTH + 1 }> = DitherTarget::new(&mut cga);
    bmp.draw(&mut display).unwrap();

    Ok(())
}
