use crate::traits::{VectorFunction};
use crate::atomics::{__Callable__, Element};
use ndarray::{Array, Ix1};

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

impl<'a, T> VectorFunction<'a, T> for Segment<T> {
    fn call(&self, t: &'a mut Array<T, Ix1>) -> () {
        todo!()
    }
}
