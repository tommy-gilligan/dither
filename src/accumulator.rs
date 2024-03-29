use embedded_graphics_core::pixelcolor::{Bgr888, Rgb888, RgbColor};

#[derive(Copy, Clone, Default, Debug)]
pub struct Accumulator(i16, i16, i16);

impl Accumulator {
    pub fn new(tuple: (i16, i16, i16)) -> Self {
        Self(tuple.0, tuple.1, tuple.2)
    }
}

impl<C> From<C> for Accumulator
where
    C: RgbColor,
{
    fn from(value: C) -> Self {
        Self(value.r().into(), value.g().into(), value.b().into())
    }
}

impl core::ops::AddAssign for Accumulator {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl core::ops::Sub for Accumulator {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl core::ops::Mul<i16> for Accumulator {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl core::ops::Shr<i16> for Accumulator {
    type Output = Self;

    fn shr(self, rhs: i16) -> Self {
        Self(self.0 >> rhs, self.1 >> rhs, self.2 >> rhs)
    }
}

// TODO: find a way to do this generically
// perhaps macro for now
impl core::convert::From<Accumulator> for Rgb888 {
    fn from(val: Accumulator) -> Self {
        Rgb888::new(
            val.0.clamp(0, 255).try_into().unwrap_or_default(),
            val.1.clamp(0, 255).try_into().unwrap_or_default(),
            val.2.clamp(0, 255).try_into().unwrap_or_default(),
        )
    }
}

impl core::convert::From<Accumulator> for Bgr888 {
    fn from(val: Accumulator) -> Self {
        Bgr888::new(
            val.1.clamp(0, 255).try_into().unwrap_or_default(),
            val.2.clamp(0, 255).try_into().unwrap_or_default(),
            val.0.clamp(0, 255).try_into().unwrap_or_default(),
        )
    }
}
