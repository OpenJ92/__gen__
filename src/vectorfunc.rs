use crate::traits;
use crate::atomics::{ __Callable__ };
use ndarray::{ Array, Ix1 };

#[derive(Debug, PartialEq, PartialOrd)]
pub struct VectorFunc<'a, T> {
    pub func: fn(&'a mut Array<T, Ix1>) -> &'a mut Array<T, Ix1>,
    pub callparam: fn(&'a mut Array<T, Ix1>) -> &'a mut Array<T, Ix1>,
}

impl<'a, T> traits::VectorFunction<'a, T> for VectorFunc<'a, T> {
    fn call(&self, mut t:  &'a mut Array<T, Ix1>) -> () {
        t = (self.func)((self.callparam)(t));
        ()
    }
}
