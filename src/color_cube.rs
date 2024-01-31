use embedded_graphics_core::pixelcolor::RgbColor;
use heapless::Entry;
use heapless::FnvIndexMap;

#[derive(Debug)]
pub struct ColorCube<C, const N: usize>(
    [[[C; N]; N]; N],
    FnvIndexMap<C, ((u64, u64, u64), usize), 64>,
)
where
    C: PartialEq + core::cmp::Eq + core::hash::Hash + Copy + Clone;

impl<C, const N: usize> ColorCube<C, N>
where
    C: PartialEq + core::cmp::Eq + core::hash::Hash + Copy + Clone,
{
    pub fn from<F>(f: &F) -> Result<Self, <u8 as TryFrom<usize>>::Error>
    where
        Self: Default,
        F: Fn(u8, u8, u8) -> C,
    {
        let mut result: Self = Self::default();
        for r in 0..N {
            for g in 0..N {
                for b in 0..N {
                    let r_8 = (255 / (N - 1)) as u8 * r as u8;
                    let g_8 = (255 / (N - 1)) as u8 * g as u8;
                    let b_8 = (255 / (N - 1)) as u8 * b as u8;

                    result.0[r][g][b] = f(r_8, g_8, b_8);
                }
            }
        }
        result.approximate_centers();

        Ok(result)
    }

    pub fn neighbours(&mut self, r: usize, g: usize, b: usize) -> heapless::Vec<&C, 26> {
        let mut result = heapless::Vec::<&C, 26>::new();
        for i in [-1isize, 0, 1] {
            if (i + r as isize) < N as isize && (i + r as isize) >= 0 {
                for j in [-1isize, 0, 1] {
                    if (j + g as isize) < N as isize && (j + g as isize) >= 0 {
                        for k in [-1isize, 0, 1] {
                            if (k + b as isize) < N as isize && (k + b as isize) >= 0 {
                                let _ = result.push(
                                    &self.0[(i + r as isize) as usize][(j + g as isize) as usize]
                                        [(k + b as isize) as usize],
                                );
                            }
                        }
                    }
                }
            }
        }
        result
    }

    pub fn approximate_centers(&mut self) {
        self.1.clear();
        for r in 0..N {
            for g in 0..N {
                for b in 0..N {
                    let r_8 = (255 / (N - 1)) as u8 * r as u8;
                    let g_8 = (255 / (N - 1)) as u8 * g as u8;
                    let b_8 = (255 / (N - 1)) as u8 * b as u8;

                    if let Entry::Vacant(v) = self.1.entry(self.0[r][g][b]) {
                        v.insert(((0, 0, 0), 0)).unwrap();
                    }
                    if let Entry::Occupied(mut o) = self.1.entry(self.0[r][g][b]) {
                        let b = o.get_mut();

                        *b = (
                            (
                                b.0 .0 + r_8 as u64,
                                b.0 .1 + g_8 as u64,
                                b.0 .2 + b_8 as u64,
                            ),
                            b.1 + 1,
                        );
                    }
                }
            }
        }
        for (_, val) in self.1.iter_mut() {
            *val = (
                (
                    val.0 .0 / val.1 as u64,
                    val.0 .1 / val.1 as u64,
                    val.0 .2 / val.1 as u64,
                ),
                val.1,
            );
        }
    }

    fn center(&self, color: C) -> (u64, u64, u64) {
        self.1.get(&color).unwrap().0
    }

    pub fn with_error<I>(&self, index: I) -> (C, (i16, i16, i16))
    where
        I: RgbColor,
        C: Copy + core::hash::Hash + Copy + Clone,
    {
        let r: i16 = index.r().into();
        let g: i16 = index.g().into();
        let b: i16 = index.b().into();

        (
            self[index],
            (
                r - self.center(self[index]).0 as i16,
                g - self.center(self[index]).1 as i16,
                b - self.center(self[index]).2 as i16,
            ),
        )
    }
}

impl<I, C, const N: usize> core::ops::Index<I> for ColorCube<C, N>
where
    I: RgbColor,
    C: PartialEq + core::cmp::Eq + core::hash::Hash + Copy + Clone,
{
    type Output = C;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index.r() as usize / (256 / N)][index.g() as usize / (256 / N)]
            [index.b() as usize / (256 / N)]
    }
}

impl<C, const N: usize> Default for ColorCube<C, N>
where
    C: Default + Copy + PartialEq + core::cmp::Eq + core::hash::Hash + Copy + Clone,
{
    fn default() -> Self {
        ColorCube(
            [[[C::default(); N]; N]; N],
            FnvIndexMap::<C, ((u64, u64, u64), usize), 64>::new(),
        )
    }
}

#[cfg(test)]
mod test {
    use embedded_graphics::pixelcolor::Rgb888;

    extern crate std;
    use super::*;

    #[derive(Eq, Hash, Debug, PartialEq, Default, Copy, Clone)]
    enum Color {
        #[default]
        White,
        Black,
        Red,
        Green,
        Blue,
        Yellow,
        Magenta,
        Cyan,
    }

    #[test]
    fn test_cube() {
        let color_cube: ColorCube<Color, 2> =
            ColorCube::from(&|r, g, b| match (r > 0x80, g > 0x80, b > 0x80) {
                (true, true, true) => Color::White,
                (false, false, false) => Color::Black,
                (true, false, false) => Color::Red,
                (false, true, false) => Color::Green,
                (false, false, true) => Color::Blue,
                (true, true, false) => Color::Yellow,
                (true, false, true) => Color::Magenta,
                (false, true, true) => Color::Cyan,
            })
            .unwrap();

        assert_eq!(color_cube.0[0][0][0], Color::Black);
        assert_eq!(color_cube.0[1][1][1], Color::White);
        assert_eq!(color_cube.0[1][0][0], Color::Red);
        assert_eq!(color_cube.0[0][1][0], Color::Green);
        assert_eq!(color_cube.0[0][0][1], Color::Blue);
        assert_eq!(color_cube.0[1][1][0], Color::Yellow);
        assert_eq!(color_cube.0[1][0][1], Color::Magenta);
        assert_eq!(color_cube.0[0][1][1], Color::Cyan);
    }

    #[test]
    fn test_cube_index() {
        let color_cube: ColorCube<Color, 2> =
            ColorCube::from(&|r, g, b| match (r > 0x80, g > 0x80, b > 0x80) {
                (true, true, true) => Color::White,
                (false, false, false) => Color::Black,
                (true, false, false) => Color::Red,
                (false, true, false) => Color::Green,
                (false, false, true) => Color::Blue,
                (true, true, false) => Color::Yellow,
                (true, false, true) => Color::Magenta,
                (false, true, true) => Color::Cyan,
            })
            .unwrap();

        assert_eq!(color_cube[Rgb888::new(0x00, 0x00, 0x00)], Color::Black);
        assert_eq!(color_cube[Rgb888::new(0xff, 0xff, 0xff)], Color::White);
        assert_eq!(color_cube[Rgb888::new(0xff, 0x00, 0x00)], Color::Red);
        assert_eq!(color_cube[Rgb888::new(0x00, 0xff, 0x00)], Color::Green);
        assert_eq!(color_cube[Rgb888::new(0x00, 0x00, 0xff)], Color::Blue);
        assert_eq!(color_cube[Rgb888::new(0xff, 0xff, 0x00)], Color::Yellow);
        assert_eq!(color_cube[Rgb888::new(0xff, 0x00, 0xff)], Color::Magenta);
        assert_eq!(color_cube[Rgb888::new(0x00, 0xff, 0xff)], Color::Cyan);
    }

    #[test]
    fn test_error() {
        let color_cube: ColorCube<Color, 2> =
            ColorCube::from(&|r, g, b| match (r > 0x80, g > 0x80, b > 0x80) {
                (true, true, true) => Color::White,
                (false, false, false) => Color::Black,
                (true, false, false) => Color::Red,
                (false, true, false) => Color::Green,
                (false, false, true) => Color::Blue,
                (true, true, false) => Color::Yellow,
                (true, false, true) => Color::Magenta,
                (false, true, true) => Color::Cyan,
            })
            .unwrap();

        assert_eq!(
            color_cube.with_error(Rgb888::new(0x01, 0x01, 0x01)),
            (Color::Black, (1, 1, 1))
        );
        assert_eq!(
            color_cube.with_error(Rgb888::new(0xf0, 0xf0, 0xf0)),
            (Color::White, (-15, -15, -15))
        );
    }
}
