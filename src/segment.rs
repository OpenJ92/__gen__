use crate::traits::{VectorFunction};
use crate::atomics::{__Callable__, Element};
use ndarray::{Ix1, IxDyn, Array};

use num_traits::real::Real;

#[derive(Clone, Debug, PartialEq,)]
pub struct Segment<T> {
    pub start: Array<T, Ix1>,
    pub end: Array<T, Ix1>,
    pub callparam: fn(Array<T, Ix1>) -> Array<T, Ix1>,
}

// impl<T> VectorFunction<T> for Segment<T>
// where
//     T: Real + Copy,
// {
//     // Like Atom::Point
//     fn call(&self, t: Vec<T>) -> Vec<T> {
//         let t: T = match &t[..] {
//             [t, ..] => *t,
//             _ => panic!(),
//         };
// 
//         let startzend = self.start
//             .iter()
//             .zip(self.end.iter())
//             .map(|u| *u.1 - *u.0);
// 
//         return self.start
//             .iter()
//             .zip(startzend)
//             .map(|u| *u.0 + (self.callparam)(t) * u.1)
//             .collect();
//     }
// }

impl<'a, T> VectorFunction<'a, T> for Segment<T> where T: Real
{
    fn call(&self, t: &'a mut Array<T, IxDyn>) -> () {
        // We have to look into how to take the given element t, which
        // initially exists on the heap, and mutate it into the new form.
        // How might we do this?
        *t = Array::zeros((3, 4, 5)).into_dyn();
        ()
    }
}
