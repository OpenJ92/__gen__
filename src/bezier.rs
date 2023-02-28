use crate::traits::{VectorFunction};
use crate::atomics::__Callable__;

use ndarray::{ Array, IxDyn, Ix1 };


// This will be the first implementation we carry out here. 
// Remember to implement it as you have it in your blog post.
struct Bezier<T> {
    pub control_points: Array<T, IxDyn>,
    pub collape_axes: Array<T, Ix1>,
    pub callparam: fn(Array<T, Ix1>) -> Array<T, Ix1>,
}

impl<'a, T> VectorFunction<'a, T> for Bezier<T> {
    fn call(&self, t: &'a mut Array<T, IxDyn>) -> () {
        todo!()
    }
}
