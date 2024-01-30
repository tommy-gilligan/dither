use embedded_graphics_core::pixelcolor::{RgbColor, Rgb888};

#[derive(Debug)]
struct ColorCube<C, const N: usize>([[[C; N]; N]; N]);

impl <C, const N: usize>ColorCube<C, N> {
    fn from<F>(f: &F) -> Result<Self, <u8 as TryFrom<usize>>::Error> where Self: Default, F: Fn(u8, u8, u8) -> C {
        let mut result: Self = Self::default();
         for r in 0..N {
             for g in 0..N {
                 for b in 0..N {
                    result.0[r][g][b] = f(r.try_into()?, g.try_into()?, b.try_into()?);
                }
            }
        }
        Ok(result)
    }
}

impl <I, C, const N: usize>core::ops::Index<I> for ColorCube<C, N> where I: RgbColor {
    type Output = C;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index.r() as usize / N][index.g() as usize / N][index.b() as usize / N]
    }
}

impl <C, const N: usize>Default for ColorCube<C, N> where C: Default + Copy {
    fn default() -> Self {
        ColorCube([[[C::default(); N]; N]; N])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cube() {
        let color_cube: ColorCube<bool, 16> = ColorCube::from(&|r, _, _| r % 2 == 0).unwrap();

        assert_eq!(color_cube[Rgb888::new(0x10, 0, 0)], false);
        assert_eq!(color_cube[Rgb888::new(0x00, 0, 0)], true);
    }
}
