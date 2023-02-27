use crate::atomics::__Callable__;
use ndarray::Array;
use ndarray::IxDyn;


// This will be the first implementation we carry out here. 
// Remember to implement it as you have it in your blog post.
struct Bezier<'a, B> {
    pub control_points: Array<IxDyn, B>,
    pub callparam: fn(__Callable__<'a, B>) -> __Callable__<'a, B>,
}
