use core::ops::AddAssign;
extern crate std;

#[derive(Debug, PartialEq)]
pub struct WrappingVec<'a, X, I, F, Y, const N: usize>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X>,
    I: Iterator<Item = X>,
    F: Fn(X) -> (Y, X)
{
    v: [X; N],
    write_index: usize,
    read_index: usize,
    kernel: &'a [(isize, X)],
    source_pixels: I,
    conversion: F,
    last_row: usize
}

impl<'a, X, I, F, Y, const N: usize> WrappingVec<'a, X, I, F, Y, N>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X>,
    I: Iterator<Item = X>,
    F: Fn(X) -> (Y, X)
{
    pub fn new(
        mut source_pixels: I,
        kernel: &'a [(isize, X)],
        conversion: F
    ) -> Self
    where
        I: Iterator<Item = X>,
    {
        let mut v = [Default::default(); N];

        for item in v.iter_mut().take(N) {
            *item = source_pixels.next().unwrap();
        }

        Self { v, read_index: 0, write_index: 0, kernel, source_pixels, conversion, last_row: 0 }
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

impl <'a, X, I, F, Y, const N: usize> Iterator for WrappingVec<'a, X, I, F, Y, N>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X>,
    I: Iterator<Item = X>,
    F: Fn(X) -> (Y, X)
{
    type Item = Y;

    fn next(&mut self) -> Option<Self::Item> {
        let (value, error) = (self.conversion)(self.v[self.read_index]);

        self.advance_read_index();

        for (x_offset, coefficient) in self.kernel {
            let index = self.read_index as isize + x_offset;

            if index < N as isize && index >= 0 {
                self.v[index as usize] += error * *coefficient;
            }
        }

        if let Some(source_pixel) = self.source_pixels.next() {
            self.v[self.write_index] = source_pixel;
            self.advance_write_index();
        } else {
            if self.last_row == N {
                return None
            }
            self.last_row += 1;
        }

        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const GREYS: [i16; 5] = [
        0x00,
        0x40,
        0x80,
        0xC0,
        0xF0
    ];

    fn quantize_ten(a: i16) -> (i16, i16) {
        let value = GREYS.into_iter().min_by_key(|ai| {
            (*ai).abs_diff(a)
        }).unwrap();

        (value, 10)
    }

    #[test]
    fn test_wrapping_vec_x_range() {
        let a: [i16; 8] = [
            1, 30, 3, 90,
            10, 6, 7, 8,
        ];
        let mut wv: WrappingVec<i16, _, _, _, 4> = WrappingVec::new(
            a.into_iter(),
            &[(0, 3)],
            &quantize_ten
        );

        assert_eq!(wv.next(), Some(0));
        assert_eq!(wv.next(), Some(64));
        assert_eq!(wv.next(), Some(64));
        assert_eq!(wv.next(), Some(128));

        wv.next();
        wv.next();
        wv.next();
        wv.next();

        assert_eq!(wv.next(), None);
    }
}
