use core::ops::{Index, IndexMut};

// weirdly carries N cells of v PLUS an extra v_n cell
// this is a mechanism to hold N+1 without resorting to unstable const expr
// complicates logic but at least it's somewhat hidden/isolated
pub struct WrappingVec<X, const N: usize>
where
    X: Default + Copy,
{
    v: [X; N],
    v_n: X,
    cursor: usize,
}

impl<X, const N: usize> WrappingVec<X, N>
where
    X: Default + Copy,
{
    pub fn new<I>(source_pixels: &mut I) -> Self
    where
        I: Iterator<Item = X>,
    {
        let mut v = [Default::default(); N];

        for item in v.iter_mut().take(N) {
            *item = source_pixels.next().unwrap();
        }
        let v_n: X = source_pixels.next().unwrap();

        Self { v, v_n, cursor: 0 }
    }

    pub fn push(&mut self, item: X) {
        if self.cursor == N {
            self.v_n = item;
        } else {
            self.v[self.cursor] = item;
        }
        self.cursor = (self.cursor + 1) % (N + 1);
    }
}

impl<X, const N: usize> Index<usize> for WrappingVec<X, N>
where
    X: Default + Copy,
{
    type Output = X;

    fn index(&self, index: usize) -> &Self::Output {
        let i = (self.cursor + index) % (N + 1);

        if i == N {
            &self.v_n
        } else {
            &self.v[i]
        }
    }
}

impl<X, const N: usize> IndexMut<usize> for WrappingVec<X, N>
where
    X: Default + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = (self.cursor + index) % (N + 1);

        if i == N {
            &mut self.v_n
        } else {
            &mut self.v[i]
        }
    }
}

#[test]
fn test_wrapping_vec() {
    let a: [u8; 6] = [1, 2, 3, 4, 5, 6];
    let mut iter = a.into_iter();
    let mut wv: WrappingVec<u8, 4> = WrappingVec::new(&mut iter);

    assert_eq!(wv[0], 1);
    assert_eq!(wv[1], 2);
    assert_eq!(wv[2], 3);
    assert_eq!(wv[3], 4);
    assert_eq!(wv[4], 5);
    assert_eq!(wv[5], 1);

    wv.push(9);
    assert_eq!(wv[0], 2);
    assert_eq!(wv[1], 3);
    assert_eq!(wv[2], 4);
    assert_eq!(wv[3], 5);
    assert_eq!(wv[4], 9);
    assert_eq!(wv[5], 2);
}
