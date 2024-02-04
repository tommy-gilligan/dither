#[derive(Default, Copy, Clone)]
pub struct Vector<T>(pub T, pub T, pub T)
where
    T: Clone;

impl<T> Vector<T>
where
    T: Clone,
{
    pub fn from_num(t: T) -> Self {
        Self(t.clone(), t.clone(), t.clone())
    }
}

use fixed::types::extra::U16;
type Num = fixed::FixedI32<U16>;

impl core::ops::AddAssign for Vector<Num> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl core::ops::Mul<Num> for Vector<Num> {
    type Output = Self;

    fn mul(self, rhs: Num) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl core::ops::Sub for Vector<Num> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics_core::pixelcolor::{Rgb888, RgbColor};
#[cfg(feature = "embedded-graphics")]
impl Vector<Num> {
    pub fn from_rgb888(t: Rgb888) -> Self {
        Self(
            Num::from_num(t.r()),
            Num::from_num(t.g()),
            Num::from_num(t.b()),
        )
    }
}
