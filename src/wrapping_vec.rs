use core::ops::AddAssign;

// weirdly carries N cells of v PLUS an extra v_n cell
// this is a mechanism to hold N+1 without resorting to unstable const expr
// complicates logic but at least it's somewhat hidden/isolated
#[derive(Debug, PartialEq)]
pub struct WrappingVec<'a, X, const N: usize>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X> + core::ops::Div<Output = X>
{
    v: [X; N],
    v_n: X,
    write_index: usize,
    read_index: usize,
    kernel: &'a [(usize, (X, X))],
}

impl<'a, X, const N: usize> WrappingVec<'a, X, N>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X> + core::ops::Div<Output = X>
{
    pub fn new<I>(source_pixels: &mut I, kernel: &'a [(usize, (X, X))]) -> Self
    where
        I: Iterator<Item = X>,
    {
        let mut v = [Default::default(); N];

        for item in v.iter_mut().take(N) {
            *item = source_pixels.next().unwrap();
        }
        let v_n: X = source_pixels.next().unwrap_or_default();

        Self { v, v_n, read_index: 0, write_index: 0, kernel }
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

    pub fn apply_kernel(&mut self, error: X, horizon: X) {
        for (index_offset, (mul, div)) in self.kernel {
            let index = self.read_index + index_offset;
            if index == N {
                self.v_n += error * *mul;
            } else {
                self.v[index] += (error * *mul) / *div;
            }

            self.push(horizon);
        }
    }
}

impl <'a, X, const N: usize> Iterator for WrappingVec<'a, X, N>
where
    X: Default + Copy + AddAssign + core::ops::Mul<Output = X> + core::ops::Div<Output = X>
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
    let mut iter = a.into_iter();
    let mut wv: WrappingVec<i16, 4> = WrappingVec::new(&mut iter, &[]);

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
    let mut iter = a.into_iter();
    let mut wv: WrappingVec<i16, 4> = WrappingVec::new(&mut iter, &[(1, (2, 1))]);

    assert_eq!(wv.next(), Some(1));
    assert_eq!(wv.next(), Some(2));
    assert_eq!(wv.next(), Some(3));
    assert_eq!(wv.next(), Some(4));
    assert_eq!(wv.next(), Some(5));
    assert_eq!(wv.next(), Some(1));

    let horizon = iter.next();
    assert_eq!(horizon, Some(6));
    wv.apply_kernel(10, horizon.unwrap());

    assert_eq!(wv.next(), Some(2));
    assert_eq!(wv.next(), Some(23));
    assert_eq!(wv.next(), Some(4));
    assert_eq!(wv.next(), Some(5));
    assert_eq!(wv.next(), Some(6));
    assert_eq!(wv.next(), Some(2));
}

#[test]
fn test_wrapping_vec_floyd_steinberg() {
    let a: [i16; 10] = [
        1, 2, 3, 4,
        5, 6, 7, 8,
        9, 10,
    ];
    let mut iter = a.into_iter();
    let mut wv: WrappingVec<i16, 4> = WrappingVec::new(
        &mut iter,
        &[
            (0, (7, 16)),
            // (4 - 2, (3, 16)),
            // (4 - 1, (5, 16)),
            // (4, (1, 16)),
        ]
    );

    assert_eq!(wv.next(), Some(1));
    wv.apply_kernel(10, iter.next().unwrap());

    assert_eq!(wv.next(), Some(6));
    wv.apply_kernel(10, iter.next().unwrap());

    assert_eq!(wv.next(), Some(7));
    wv.apply_kernel(10, iter.next().unwrap());

    assert_eq!(wv.next(), Some(8));
    wv.apply_kernel(10, iter.next().unwrap());
}
