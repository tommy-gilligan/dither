use crate::wrapping_vec::WrappingVec;

#[derive(Debug, PartialEq)]
struct Dither<'a, A, B, FAX, X> where B: Iterator<Item = A>, FAX: Fn(A) -> X, A: Clone + Default + Copy {
    source: B,
    closest_color: FAX,
    // assume normalized to 1/16 for now
    kernel: &'a [(usize, usize, i16)],
    width: usize,
    window: crate::wrapping_vec::WrappingVec<A, 1>
}

impl <'a, A, B, FAX, X>Dither<'a, A, B, FAX, X> where B: Iterator<Item = A>, FAX: Fn(A) -> X, A: Clone + Default + Copy {
    pub fn new(mut source: B, closest_color: FAX, kernel: &'a [(usize, usize, i16)], width: usize) -> Self {
        let window = WrappingVec::<A, 1>::new(&mut source);

        Dither {
            source,
            closest_color,
            kernel,
            width,
            window
        }
    }
}

impl <'a, A, B, FAX, X>Iterator for Dither<'a, A, B, FAX, X> where B: Iterator<Item = A>, FAX: Fn(A) -> X, A: Clone + Default + Copy {
    type Item = X;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|a| (self.closest_color)(a))
    }
}

const GREYS: [u8; 5] = [
    0x00,
    0x40,
    0x80,
    0xC0,
    0xF0
];

fn quantize(a: u8) -> (u8, i16) {
    let value = GREYS.into_iter().min_by_key(|ai| {
        (*ai).abs_diff(a)
    }).unwrap();

    (value, a as i16 - value as i16)
}

#[test]
fn test_one_item_no_error() {
    let source = [0x80];
    let mut dither = Dither::new(source.into_iter(), quantize, &[], 1);

    assert_eq!(dither.next(), Some((0x80, 0x00)));
    assert_eq!(dither.next(), None);
}

#[test]
fn test_one_item_some_error() {
    let source = [0x90];
    let mut dither = Dither::new(source.into_iter(), quantize, &[], 1);

    assert_eq!(dither.next(), Some((0x80, 0x10)));
    assert_eq!(dither.next(), None);
}

#[test]
fn test_two_item_no_error() {
    let source = [0x80, 0x40];
    let mut dither = Dither::new(source.into_iter(), quantize, &[], 1);

    assert_eq!(dither.next(), Some((0x80, 0x00)));
    assert_eq!(dither.next(), Some((0x40, 0x00)));
    assert_eq!(dither.next(), None);
}

#[test]
fn test_two_item_some_error() {
    let source = [0x90, 0x5A];
    let mut dither = Dither::new(source.into_iter(), quantize, &[(1, 0, 7)], 2);

    assert_eq!(dither.next(), Some((0x80, 0x10)));
    assert_eq!(dither.next(), Some((0x80, 0x1F)));
    assert_eq!(dither.next(), None);
}
