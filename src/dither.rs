// use crate::wrapping_vec::WrappingVec;
// use core::ops::AddAssign;
// 
// #[derive(Debug, PartialEq)]
// struct Dither<A, B, FAX, X, QE> where B: Iterator<Item = A>, FAX: Fn(A) -> (X, QE), A: Clone + Default + Copy + AddAssign {
//     source: B,
//     closest_color: FAX,
//     // assume normalized to 1/16 for now
//     width: usize,
//     window: crate::wrapping_vec::WrappingVec<A, 1>
// }
// 
// impl <A, B, FAX, X, QE>Dither<A, B, FAX, X, QE> where B: Iterator<Item = A>, FAX: Fn(A) -> (X, QE), A: Clone + Default + Copy + AddAssign {
//     pub fn new(mut source: B, closest_color: FAX, kernel: &[(usize, i16)], width: usize) -> Self {
//         let window = WrappingVec::<A, 1>::new(&mut source, kernel);
// 
//         Dither {
//             source,
//             closest_color,
//             width,
//             window
//         }
//     }
// }
// 
// impl <A, B, FAX, X>Iterator for Dither<A, B, FAX, X, i16> where B: Iterator<Item = A>, FAX: Fn(A) -> (X, i16), A: Clone + Default + Copy + AddAssign {
//     type Item = X;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.window.next() {
//             Some(popped) => {
//                 let (dithered_color, quantization_error): (X, i16) = (self.closest_color)(self.window.next().unwrap().into());
//                 // self.window.apply_kernel(
//                 //     self.kernel,
//                 //     quantization_error,
//                 // );
// 
//                 // self.window[1] += (quantization_error * 7) >> 4;
//                 // self.window[self.width - 1] += (quantization_error * 3) >> 4;
//                 // self.window[self.width] += (quantization_error * 5) >> 4;
//                 // self.window[self.width + 1] += (quantization_error) >> 4;
// 
//                 // self.window.push(horizon_pixel);
// 
//                 Some(dithered_color)
//             },
//             None => None
//         }
// 
//     }
// }
// 
// const GREYS: [u8; 5] = [
//     0x00,
//     0x40,
//     0x80,
//     0xC0,
//     0xF0
// ];
// 
// fn quantize(a: u8) -> (u8, i16) {
//     let value = GREYS.into_iter().min_by_key(|ai| {
//         (*ai).abs_diff(a)
//     }).unwrap();
// 
//     (value, a as i16 - value as i16)
// }
// 
// #[test]
// fn test_one_item_no_error() {
//     let source = [0x80];
//     let mut dither = Dither::new(source.into_iter(), quantize, &[], 1);
// 
//     assert_eq!(dither.next(), Some(0x80));
//     assert_eq!(dither.next(), None);
// }
// 
// #[test]
// fn test_one_item_some_error() {
//     let source = [0x90];
//     let mut dither = Dither::new(source.into_iter(), quantize, &[], 1);
// 
//     assert_eq!(dither.next(), Some(0x80));
//     assert_eq!(dither.next(), None);
// }
// 
// #[test]
// fn test_two_item_no_error() {
//     let source = [0x80, 0x40];
//     let mut dither = Dither::new(source.into_iter(), quantize, &[], 1);
// 
//     assert_eq!(dither.next(), Some(0x80));
//     assert_eq!(dither.next(), Some(0x40));
//     assert_eq!(dither.next(), None);
// }
// 
// #[test]
// fn test_two_item_some_error() {
//     let source = [0x90, 0x5A];
//     let mut dither = Dither::new(source.into_iter(), quantize, &[(1, 7)], 2);
// 
//     assert_eq!(dither.next(), Some(0x80));
//     assert_eq!(dither.next(), Some(0x80));
//     assert_eq!(dither.next(), None);
// }
