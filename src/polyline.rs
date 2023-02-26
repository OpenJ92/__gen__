use crate::traits;
use crate::traits::VectorFunction;
use crate::segment::Segment;

use ndarray::{Array, Ix1};
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use num_traits::real::Real;
use num_traits::zero;
use num_traits::one;

#[derive(Debug, PartialEq)]
pub struct PolyLine<'a, B> {
    pub segments: &'a Array<Segment<B>, Ix1>,
    pub callparam: fn(B) -> B,
}

// impl<'a, B> traits::VectorFunction<B> for PolyLine<'a, B>
// where
//     B: Copy + Real + FromPrimitive + ToPrimitive, 
// {
//     // In some sense, this will be the implementation of call_point. All other
//     // methods on the trait AVectorFunc
//     fn call(&self, t: Vec<B>) -> Vec<B> {
//         // Capture Real number to apply to VectorFunction::Polyline
//         let t: B = match &t[..] {
//             [t, ..] => *t,
//             _ => panic!(),
//         };
// 
//         // Update Real number according to paraemterization. Query length of
//         // segments. Thier product whole.frac will be the vector accessor, whole, 
//         // and call, frac, to the appropriate Segment inside.
//         let t        : B     = (self.callparam)(t);
//         let length   : usize = self.segments.len();
//         let location : B     = t * B::from_usize(length).unwrap();
// 
//         // Check location to be interior to the size of the vector.
//         let (whole, frac) = 
//             if location >= B::from_usize(length).unwrap() {
//                 let frac  : B     = one(); 
//                 let whole : usize = length - one::<usize>();
//                 (whole, frac)
//             } else if location < zero() {
//                 let frac  : B     = zero();
//                 let whole : usize = zero();
//                 (whole, frac)
//             } else {
//                 let frac  : B     = location.fract(); 
//                 let whole : usize = location.floor().to_usize().unwrap();
//                 (whole, frac)
//             };
// 
//         // Capture the appropriate segment from segments and call on frac.
//         return self.segments[whole].call(vec!(frac))
//     }
// }

// impl<'a, T: ndarray::Dimension> traits::AVectorFunction<T> for PolyLine<'a, T> {
//     // There might be one more construction of the "VFF" Traits. We might want to have
//     // the call function typed like the following:
//     //  
//     //      fn call(&self, t: mut &Atom<T>) -> ()
//     //
//     //  where we simply mutate the memory at the given address. The Array components
//     //  inside the Atom types are heap allocated. Then a composition of functions applied
//     //  to the heap will update the components inplace.
//     fn call(&self, t: &Atom<T>) -> Result<&Atom<T>, ()> {
//         let res: Result<&Atom<T>, ()> = match t {
//             scalar@Atom::Scalar { .. } => self.call_scalar(scalar),
//             point@Atom::Point { .. } => self.call_point(point),
//             line@Atom::Line { .. } => self.call_line(line),
//             triangle@Atom::Triangle { .. } => self.call_triangle(triangle)
//         };
//         res
//     }
// 
//     fn call_scalar(&self, t: &Atom<T>) -> Result<&Atom<T>, ()> {
//         Err(())
//     }
//     fn call_point(&self, t: &Atom<T>) -> Result<&Atom<T>, ()> {
//         Err(())
//     }
//     fn call_line(&self, t: &Atom<T>)  -> Result<&Atom<T>, ()>{
//         let res: Result<&Atom<T>, ()> = match t {
//             Atom::Line { start: start, end: end } 
//               => { let resstart: Result<&Array<Ix1, T>, ()>
//                             = match self.call_point(&Atom::Point {point: start}) {
//                                       Ok(Atom::Point { point: arr }) => Ok(arr), 
//                                       Ok(_)   => Err(()), 
//                                       Err(..) => Err(())
//                             };
//                    let resend: Result<&Array<Ix1, T>, ()>  
//                           = match self.call_point(&Atom::Point {point: end}) {
//                                       Ok(Atom::Point { point: arr }) => Ok(arr),  
//                                       Ok(_)   => Err(()), 
//                                       Err(..) => Err(())
//                           };
//                    let val = Atom::Line { start: resstart? , end: resend? };
//                    Err(()) 
//                  },
//             _   => Err(()),
//         };
//         res
//     }
//     fn call_triangle(&self, t: &Atom<T>)  -> Result<&Atom<T>, ()>  {
//             Err(())
//     }
// }
