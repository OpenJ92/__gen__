use crate::atomics::{__Point__, Atom, AtomA, __Callable__};

pub trait Atomic<T> : Clone {}
pub trait Atomic_ : Clone {}
pub trait SampleMethod<T> {
    // As we continue to develop this software, this form should become
    // fn values(&self) -> Vec<impl Atomic>. Atomic should be exportable
    // forms and callable forms. With that, the VectorFunction required
    // function call should be fn call(&self, t: impl Atomic> -> impl Atomic.
    fn values(&self) -> Vec<Vec<T>>;
}
pub trait VectorFunction<T> {
    // fn call<T: Atomic>(&self, t: T) -> T;
    fn call(&self, t: Vec<T>) -> Vec<T>;
    // fn sample(&self, sample_method: impl SampleMethod<T>) -> Vec<Vec<T>> {
    //     return sample_method.values().iter().map(move |t| self.call(*t.clone())).collect()
    // }
}
pub trait Exportable { 
    fn export_(&self) -> String;
}
pub trait Random<T> {
    fn rand(&self) -> Self;
}

// pub trait Sample_Method<T: Atomic_> {
//     fn values(&self) -> Vec<T>;
// }
// pub trait Vector_Function {
//     fn call<T: Atomic_>(&self, t: T) -> T;
//     fn sample<T: Atomic_>(&self, sample_method: impl Sample_Method<T>) -> Vec<T> {
//         // We're forced to use clone here which is quite upsetting. We'd like to just be able
//         // to Copy the component. Additionally, we can't declare reference functions? Why is
//         // that the case?
//         return sample_method.values().iter().map(move |t| self.call(t.clone())).collect()
//     }
// }

// pub trait ASampleMethod<T: ndarray::Dimension> {
//     fn values(&self) -> Vec<&Atom<T>>;
// }
// pub trait AVectorFunction<T: ndarray::Dimension> {
//     fn call(&self, t: &Atom<T>) -> Result<&Atom<T>, ()>;
//     fn call_scalar(&self, t: &Atom<T>) -> Result<&Atom<T>, ()>;
//     fn call_point(&self, t: &Atom<T>) -> Result<&Atom<T>, ()>;
//     fn call_line(&self, t: &Atom<T>) -> Result<&Atom<T>, ()>;
//     fn call_triangle(&self, t: &Atom<T>) -> Result<&Atom<T>, ()>;
//     
//     // fn sample(&self, sample_method: impl ASampleMethod<T>) -> Result<Vec<&Atom<T>>, ()> {
//     //     // let values     = sample_method.values();
//     //     // let mut resolv = Vec::<&Atom<T>>::new();
// 
//     //     // for value in values.iter() {
//     //     //     resolv.push(self.call(value));
//     //     // }
//     //     // return resolv;
//     //     Err(())
//     // }
// }

pub trait BSampleMethod<T: ndarray::Dimension> {
    fn values(&self) -> Vec<&__Callable__<T>>; 
}

pub trait BVectorFunction<'a, T: ndarray::Dimension> {
    fn call(&self, t: &'a mut __Callable__<T>) -> ();
    fn __call__point__(&self, t: &'a mut __Point__<T>) -> ();
    fn sample(&self, sample_method: impl BSampleMethod<T>) -> Result<__Callable__<T>, ()> {
        Err(())
    }
}
