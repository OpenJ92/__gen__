use crate::atomics::__Callable__;
use ndarray::Array;
use ndarray::IxDyn;

struct Bezier<'a, B> {
    pub control_points: Array<IxDyn, B>,
    pub callparam: fn(__Callable__<'a, B>) -> __Callable__<'a, B>,
}
