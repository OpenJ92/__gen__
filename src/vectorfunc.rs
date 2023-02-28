use crate::traits;
use crate::atomics::{ __Callable__ };
use ndarray::{ Array, Ix1, IxDyn };

#[derive(Debug, PartialEq, PartialOrd)]
pub struct VectorFunc<'a, T> {
    pub func: fn(&'a mut Array<T, IxDyn>) -> &'a mut Array<T, IxDyn>,
    pub callparam: fn(&'a mut Array<T, IxDyn>) -> &'a mut Array<T, IxDyn>,
}

impl<'a, T> traits::VectorFunction<'a, T> for VectorFunc<'a, T> {
    fn call(&self, mut t:  &'a mut Array<T, IxDyn>) -> () {
        t = (self.func)((self.callparam)(t));
        ()
    }
}
