use embedded_graphics_simulator::{
    OutputSettingsBuilder,
    SimulatorDisplay,
    Window,
};
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb888
};
use tinybmp::Bmp;
use dither::{DitherTarget, cga};

const WIDTH: usize = 256;
const HEIGHT: usize = 383;

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let size = Size::new(WIDTH as u32, HEIGHT as u32);

    let mut simulator_display = SimulatorDisplay::<Rgb888>::new(size);
    let mut cga: cga::FakeCGA<SimulatorDisplay::<Rgb888>> = cga::FakeCGA::new(&mut simulator_display);
    let mut display: DitherTarget<'_, _, { WIDTH + 1 }> = DitherTarget::new(&mut cga);
    bmp.draw(&mut display).unwrap();

    Window::new(
        "Mona Lisa",
        &OutputSettingsBuilder::new().build()
    ).show_static(&simulator_display);
    Ok(())
}
