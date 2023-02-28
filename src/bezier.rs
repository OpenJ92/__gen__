use crate::traits::{VectorFunction};
use crate::atomics::__Callable__;

use ndarray::{ Array, IxDyn, Ix1, Axis };
use num_traits::real::Real;

// This will be the first implementation we carry out here. 
// Remember to implement it as you have it in your blog post.
struct Bezier<T> {
    pub control_points: Array<T, IxDyn>,
    pub collapse_axes: Array<T, Ix1>,
    pub callparam: fn(Array<T, Ix1>) -> Array<T, Ix1>,
    // It's seeming like we're going to have to store the partial computation here or we'll have to
    // make new control_points, collapse_axes and callparam in a new bezier and then call the
    // updated t on it. 
}

impl<'a, T> VectorFunction<'a, T> for Bezier<T> 
where T: Real {
    fn call(&self, t: &'a mut Array<T, IxDyn>) -> () {
        let (x, xs) = t.view().split_at(Axis(0), 1);
        let (c, cs) = self.collapse_axes.view().split_at(Axis(0), 1);

        self.control_points.view().axis_iter(Axis(c.first().unwrap().to_usize().unwrap()));

        if cs.is_empty() { todo!(); }


        *t = xs.to_owned().into_dyn();
    }
}
