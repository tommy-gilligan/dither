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
