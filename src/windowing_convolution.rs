use core::ops::{AddAssign, Mul};

#[derive(Debug, PartialEq)]
pub struct WindowingConvolution<'a, X, I, F, Y, Z, const N: usize>
where
    X: Default + Copy + AddAssign + Mul<Z, Output = X>,
    I: Iterator<Item = X>,
    F: Fn(X) -> (Y, X),
    Z: Copy,
{
    v: [X; N],
    write_index: usize,
    read_index: usize,
    kernel: &'a [((isize, isize), Z)],
    source_pixels: I,
    conversion: F,
    last_row: usize,
}

impl<'a, X, I, F, Y, Z, const N: usize> WindowingConvolution<'a, X, I, F, Y, Z, N>
where
    X: Default + Copy + AddAssign + Mul<Z, Output = X>,
    I: Iterator<Item = X>,
    F: Fn(X) -> (Y, X),
    Z: Copy,
{
    pub fn new(mut source_pixels: I, kernel: &'a [((isize, isize), Z)], conversion: F) -> Self
    where
        I: Iterator<Item = X>,
    {
        let mut v = [Default::default(); N];

        for item in v.iter_mut().take(N) {
            *item = source_pixels.next().unwrap();
        }

        Self {
            v,
            read_index: 0,
            write_index: 0,
            kernel,
            source_pixels,
            conversion,
            last_row: 0,
        }
    }

    fn advance_read_index(&mut self) {
        self.read_index = if self.read_index < (N - 1) {
            self.read_index + 1
        } else {
            0
        };
    }

    fn advance_write_index(&mut self) {
        self.write_index = if self.write_index < (N - 1) {
            self.write_index + 1
        } else {
            0
        };
    }
}

impl<'a, X, I, F, Y, Z, const N: usize> Iterator for WindowingConvolution<'a, X, I, F, Y, Z, N>
where
    X: Default + Copy + AddAssign + Mul<Z, Output = X>,
    I: Iterator<Item = X>,
    F: Fn(X) -> (Y, X),
    Z: Copy,
{
    type Item = Y;

    fn next(&mut self) -> Option<Self::Item> {
        let (value, error) = (self.conversion)(self.v[self.read_index]);

        for ((x_offset, _y_offset), coefficient) in self.kernel {
            let index = self.read_index as isize + x_offset;

            if index < N as isize && index >= 0 {
                self.v[index as usize] += error * *coefficient;
            }
        }

        self.advance_read_index();

        if let Some(source_pixel) = self.source_pixels.next() {
            self.v[self.write_index] = source_pixel;
            self.advance_write_index();
        } else {
            if self.last_row == N {
                return None;
            }
            self.last_row += 1;
        }

        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use fixed::types::extra::U16;
    type Num = fixed::FixedI32<U16>;

    fn quantize_ten(a: Num) -> (Num, Num) {
        let greys: [Num; 5] = [
            Num::from_num(0x00),
            Num::from_num(0x40),
            Num::from_num(0x80),
            Num::from_num(0xC0),
            Num::from_num(0xF0),
        ];
        let value = greys
            .into_iter()
            .min_by_key(|ai| (*ai).abs_diff(a))
            .unwrap();

        (value, Num::from_num(10))
    }

    #[test]
    fn test_windowing_convolution_x_range() {
        let a: [Num; 8] = [
            Num::from_num(1),
            Num::from_num(30),
            Num::from_num(3),
            Num::from_num(90),
            Num::from_num(10),
            Num::from_num(6),
            Num::from_num(7),
            Num::from_num(8),
        ];
        let kernel = [((1, 0), Num::from_num(3))];
        let mut wv: WindowingConvolution<Num, _, _, _, _, 4> =
            WindowingConvolution::new(a.into_iter(), &kernel, &quantize_ten);

        assert_eq!(wv.next(), Some(Num::from_num(0)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));
        assert_eq!(wv.next(), Some(Num::from_num(128)));

        assert_eq!(wv.next(), Some(Num::from_num(0)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));

        assert_eq!(wv.next(), None);
    }

    #[test]
    fn test_windowing_convolution_x_2_range() {
        let a: [Num; 8] = [
            Num::from_num(1),
            Num::from_num(30),
            Num::from_num(3),
            Num::from_num(90),
            Num::from_num(10),
            Num::from_num(6),
            Num::from_num(7),
            Num::from_num(8),
        ];
        let kernel = [((2, 0), Num::from_num(3))];
        let mut wv: WindowingConvolution<Num, _, _, _, _, 4> =
            WindowingConvolution::new(a.into_iter(), &kernel, &quantize_ten);

        assert_eq!(wv.next(), Some(Num::from_num(0)));
        assert_eq!(wv.next(), Some(Num::from_num(0)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));
        assert_eq!(wv.next(), Some(Num::from_num(128)));

        assert_eq!(wv.next(), Some(Num::from_num(0)));
        assert_eq!(wv.next(), Some(Num::from_num(0)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));
        assert_eq!(wv.next(), Some(Num::from_num(64)));

        assert_eq!(wv.next(), None);
    }

    // #[test]
    // fn test_windowing_convolution_bottom_range() {
    //     let a: [Num; 8] = [
    //         Num::from_num(1),
    //         Num::from_num(30),
    //         Num::from_num(3),
    //         Num::from_num(90),

    //         Num::from_num(10),
    //         Num::from_num(6),
    //         Num::from_num(7),
    //         Num::from_num(8),
    //     ];
    //     let kernel = [((0, 1), Num::from_num(3))];
    //     let mut wv: WindowingConvolution<Num, _, _, _, 4> =
    //         WindowingConvolution::new(a.into_iter(), &kernel, &quantize_ten);

    //     assert_eq!(wv.next(), Some(Num::from_num(0)));
    //     assert_eq!(wv.next(), Some(Num::from_num(0)));
    //     assert_eq!(wv.next(), Some(Num::from_num(0)));
    //     assert_eq!(wv.next(), Some(Num::from_num(64)));

    //     assert_eq!(wv.next(), Some(Num::from_num(64)));
    //     assert_eq!(wv.next(), Some(Num::from_num(64)));
    //     assert_eq!(wv.next(), Some(Num::from_num(64)));
    //     assert_eq!(wv.next(), Some(Num::from_num(64)));

    //     assert_eq!(wv.next(), None);
    // }
}
