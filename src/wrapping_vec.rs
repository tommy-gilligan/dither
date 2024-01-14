use core::ops::{Index, IndexMut};

pub struct WrappingVec<X, const N: usize> where X: Default + Copy {
    v: [X; N],
    cursor: usize,
}

impl <X, const N: usize>WrappingVec<X, N> where X: Default + Copy {
    pub fn new<I>(source_pixels: &mut I) -> Self where I: Iterator<Item = X> {
        let mut v = [Default::default(); N];

        for item in v.iter_mut().take(N) {
            *item = source_pixels.next().unwrap();
        }

        Self { v, cursor: 0 }
    }

    pub fn push(&mut self, item: X) {
        self.v[self.cursor] = item;
        self.cursor = (self.cursor + 1) % N;
    }
}

impl <X, const N: usize>Index<usize> for WrappingVec<X, N> where X: Default + Copy {
    type Output = X;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[(self.cursor + index) % N]
    }
}

impl <X, const N: usize>IndexMut<usize> for WrappingVec<X, N> where X: Default + Copy {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(self.cursor + index) % N]
    }
}
