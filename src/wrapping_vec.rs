use core::ops::AddAssign;
extern crate std;

// weirdly carries N cells of v PLUS an extra v_n cell
// this is a mechanism to hold N+1 without resorting to unstable const expr
// complicates logic but at least it's somewhat hidden/isolated
#[derive(Debug, PartialEq)]
pub struct WrappingVec<'a, X, I, const N: usize>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X> + core::ops::Div<Output = X>,
    I: Iterator<Item = X>
{
    v: [X; N],
    v_n: X,
    write_index: usize,
    read_index: usize,
    kernel: &'a [((isize, isize), (X, X))],
    source_pixels: I
}

impl<'a, X, I, const N: usize> WrappingVec<'a, X, I, N>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X> + core::ops::Div<Output = X>,
    I: Iterator<Item = X>
{
    pub fn new(
        mut source_pixels: I,
        kernel: &'a [((isize, isize), (X, X))],
    ) -> Self
    where
        I: Iterator<Item = X>,
    {
        let mut v = [Default::default(); N];

        for item in v.iter_mut().take(N) {
            *item = source_pixels.next().unwrap();
        }
        let v_n: X = source_pixels.next().unwrap_or_default();

        Self { v, v_n, read_index: 0, write_index: 0, kernel, source_pixels }
    }

    fn push(&mut self, item: X) {
        let i = self.write_index;

        self.write_index = if self.write_index < N {
            self.write_index + 1
        } else {
            0
        };

        if i == N {
            self.v_n = item;
        } else {
            self.v[i] = item;
        }
    }

    pub fn apply_kernel(&mut self, error: X) {
        let horizon = self.source_pixels.next().unwrap();
        for ((x_offset, y_offset), (mul, div)) in self.kernel {
            let index = (self.read_index as isize + x_offset + y_offset * N as isize) % (N as isize + 1);
            // std::println!("{:?}         {:?}   {:?}", self.read_index, x_offset, y_offset);

            if index <= N as isize && index >= 0 {
                if index as usize == N {
                    self.v_n += error * *mul;
                } else {
                    self.v[index as usize] += (error * *mul) / *div;
                }
            }
        }

        self.push(horizon);
    }
}

impl <'a, X, I, const N: usize> Iterator for WrappingVec<'a, X, I, N>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X> + core::ops::Div<Output = X>,
    I: Iterator<Item = X>
{
    type Item = X;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.read_index;

        self.read_index = if self.read_index < N {
            self.read_index + 1
        } else {
            0
        };

        if i == N {
            Some(self.v_n)
        } else {
            Some(self.v[i])
        }
    }
}

#[test]
fn test_wrapping_vec() {
    let a: [i16; 6] = [1, 2, 3, 4, 5, 6];
    let iter = a.into_iter();
    let mut wv: WrappingVec<i16, _, 4> = WrappingVec::new(iter, &[]);

    assert_eq!(wv.next(), Some(1));
    assert_eq!(wv.next(), Some(2));
    assert_eq!(wv.next(), Some(3));
    assert_eq!(wv.next(), Some(4));
    assert_eq!(wv.next(), Some(5));
    assert_eq!(wv.next(), Some(1));

    wv.push(9);
    assert_eq!(wv.next(), Some(2));
    assert_eq!(wv.next(), Some(3));
    assert_eq!(wv.next(), Some(4));
    assert_eq!(wv.next(), Some(5));
    assert_eq!(wv.next(), Some(9));
    assert_eq!(wv.next(), Some(2));
}

#[test]
fn test_wrapping_vec_convolution() {
    let a: [i16; 6] = [1, 2, 3, 4, 5, 6];
    let iter = a.into_iter();
    let mut wv: WrappingVec<i16, _, 4> = WrappingVec::new(iter, &[((1, 0), (2, 1))]);

    assert_eq!(wv.next(), Some(1));
    assert_eq!(wv.next(), Some(2));
    assert_eq!(wv.next(), Some(3));
    assert_eq!(wv.next(), Some(4));
    assert_eq!(wv.next(), Some(5));
    assert_eq!(wv.next(), Some(1));

    wv.apply_kernel(10);

    assert_eq!(wv.next(), Some(2));
    assert_eq!(wv.next(), Some(23));
    assert_eq!(wv.next(), Some(4));
    assert_eq!(wv.next(), Some(5));
    assert_eq!(wv.next(), Some(6));
    assert_eq!(wv.next(), Some(2));
}

#[test]
fn test_wrapping_vec_x_range() {
    let a: [i16; 10] = [
        1, 2, 3, 4,
        5, 6, 7, 8,
        9, 10,
    ];
    let iter = a.into_iter();
    let mut wv: WrappingVec<i16, _, 4> = WrappingVec::new(
        iter,
        &[
            ((0, 0), (7, 16)),
            ((4, 0), (7, 16)),
            ((-4, 0), (7, 16)),
        ]
    );

    assert_eq!(wv.next(), Some(1));
    wv.apply_kernel(10);

    assert_eq!(wv.next(), Some(6));
    wv.apply_kernel(10);

    assert_eq!(wv.next(), Some(7));
    wv.apply_kernel(10);

    assert_eq!(wv.next(), Some(8));
    wv.apply_kernel(10);
}

#[test]
fn test_wrapping_vec_y_range() {
    let a: [i16; 10] = [
        1, 2, 3, 4,
        5, 6, 7, 8,
        9, 10,
    ];
    let iter = a.into_iter();
    let mut wv: WrappingVec<i16, _, 4> = WrappingVec::new(
        iter,
        &[
            ((0, 1), (7, 16)),
        ]
    );

    assert_eq!(wv.next(), Some(1));
    std::println!("{:?}", wv);
    // wv.apply_kernel(10, iter.next().unwrap());
    // std::println!("{:?}", wv);

    // assert_eq!(wv.next(), Some(2));
    // wv.apply_kernel(10, iter.next().unwrap());

    // assert_eq!(wv.next(), Some(3));
    // wv.apply_kernel(10, iter.next().unwrap());

    // assert_eq!(wv.next(), Some(4));
    // wv.apply_kernel(10, iter.next().unwrap());



    // assert_eq!(wv.next(), Some(9));
    // wv.apply_kernel(10, iter.next().unwrap());

    // assert_eq!(wv.next(), Some(10));
    // wv.apply_kernel(10, iter.next().unwrap());

    // assert_eq!(wv.next(), Some(11));
    // wv.apply_kernel(10, iter.next().unwrap());

    // assert_eq!(wv.next(), Some(12));
    // wv.apply_kernel(10, iter.next().unwrap());
}
