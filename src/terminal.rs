use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::Rgb888, prelude::*};
use image::{DynamicImage, Rgb};
use viuer::{print, Config, ViuError};

pub struct SimulatorDisplay {
    size: Size,
    buffer: DynamicImage,
    config: Config,
}

impl SimulatorDisplay {
    pub fn new(size: Size) -> Self {
        let config = Config {
            x: 0,
            y: 0,
            // chars
            width: None,
            height: None,
            use_kitty: true,
            ..Default::default()
        };
        let buffer = DynamicImage::new_rgb8(size.width, size.height);

        Self {
            size,
            buffer,
            config,
        }
    }
}

impl DrawTarget for SimulatorDisplay {
    type Color = Rgb888;
    type Error = ViuError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let image_buffer = self.buffer.as_mut_rgb8().unwrap();
        for pixel in pixels {
            image_buffer.put_pixel(
                pixel.0.x.try_into().unwrap(),
                pixel.0.y.try_into().unwrap(),
                Rgb(pixel.1.to_be_bytes()),
            );
        }
        print(&self.buffer, &self.config).map(|_| ())
    }
}

impl OriginDimensions for SimulatorDisplay {
    fn size(&self) -> Size {
        self.size
    }
}
