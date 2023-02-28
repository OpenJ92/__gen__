use crate::atomics::{Element, __Callable__};
use ndarray::{Array, IxDyn};

pub trait Exportable { 
    fn export_(&self) -> String;
}
pub trait Random<T> {
    fn rand(&self) -> Self;
}

pub trait VectorFunction<'a,T> {
    fn call(&self, t: &'a mut Array<T, IxDyn>) -> ();
    fn __call_dispatch__(&self, t: &'a mut __Callable__<T>) -> () {
        let res = match t {
            __Callable__::Atomic(Element::Point(p))             => {
                self.call(*p);
            },
            __Callable__::Atomic(Element::Line(p1, p2))         => {
                self.call(*p1);
                self.call(*p2);
            }, 
            __Callable__::Atomic(Element::Triangle(p1, p2, p3)) => {
                self.call(*p1);
                self.call(*p2);
                self.call(*p3);
            }, 
            __Callable__::Composite(ps) => {
                ps.iter_mut().for_each(|p| self.__call_dispatch__(*p));
            }
        };
    }
    fn sample(&self, sample_method: impl SampleMethod<T>) -> Vec<&__Callable__<T>> {
        // // error[E02777]
        // sample_method.values().iter_mut().map(|p| self.call(*p)).collect()
        todo!()
    }
}

pub trait SampleMethod<T> {
    fn values(&self) -> Vec<&mut __Callable__<T>>; 
}
