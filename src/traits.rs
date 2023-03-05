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
                self.call(&mut *p);
            },
            __Callable__::Atomic(Element::Line(p1, p2))         => {
                self.call(&mut *p1);
                self.call(&mut *p2);
            }, 
            __Callable__::Atomic(Element::Triangle(p1, p2, p3)) => {
                self.call(&mut *p1);
                self.call(&mut *p2);
                self.call(&mut *p3);
            }, 
            __Callable__::Composite(ps) => {
                ps.iter_mut().for_each(|p| self.__call_dispatch__(&mut *p));
            }
        };
        ()
    }
}

pub trait Sample<'a, T> {
    fn values(&self) -> Vec<&'a mut __Callable__<T>>; 
}
