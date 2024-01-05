use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::{Rgb888, RgbColor},
    prelude::*,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::{Bmp, RawBmp};

fn main() -> Result<(), core::convert::Infallible> {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let abmp = RawBmp::from_slice(include_bytes!("./mona_lisa.bmp")).unwrap();
    let size = abmp.header().image_size;
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(size.width, size.height));

    let iter = dither::dither(bmp.pixels().map(|c| c.1), size, dither::closest);
    let pixels: Vec<u8> = iter.flat_map(|c| [c.r(), c.g(), c.b()]).collect();
    Image::new(&ImageRaw::<Rgb888>::new(&pixels, size.width), Point::zero()).draw(&mut display)?;
    Window::new("Hello World", &OutputSettingsBuilder::new().build()).show_static(&display);

    Ok(())
}
