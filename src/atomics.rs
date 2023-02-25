use ndarray::{ Array, Ix0, Ix1 };

#[derive(Clone, Copy, Debug)]
pub enum Atom<'a, T: ndarray::Dimension> {
    Scalar   { scalar: T }, 
    Point    { point:  &'a Array<Ix1, T> }, 
    Line     { start:  &'a Array<Ix1, T>, end: &'a Array<Ix1, T> },
    Triangle { points: [&'a Array<Ix1, T>;3] }, 
}

// Perhaps this is a better formulation 
// pub struct __Point__<'a, T: ndarray::Dimension> {
//     pub point:  &'a mut Array<Ix1, T> 
// }
// 
// pub enum Element<'a, T: ndarray::Dimension> {
//     Scalar(&'a mut Array<Ix0, T>),
//     Point(&'a mut __Point__<'a, T>),
//     Line(&'a mut __Point__<'a, T>, &'a mut __Point__<'a, T>),
//     Triangle(&'a mut __Point__<'a, T>,&'a mut __Point__<'a, T>, &'a mut __Point__<'a, T>)
// }

pub enum Element<'a, T> {
    Point(&'a mut Array<T, Ix1>),
    Line(&'a mut Array<T, Ix1>, &'a mut Array<T, Ix1>),
    Triangle(&'a mut Array<T, Ix1>, &'a mut Array<T, Ix1>, &'a mut Array<T, Ix1>)
}
pub enum __Callable__<'a, T> {
    Atomic(&'a mut Element<'a, T>),
    Composite(Vec<&'a mut __Callable__<'a, T>>)
}
// Here we're looking to build a set of vector functions which act upon a heap allocated
// set of Atom(A)s. 
// 
// i.e. Apply(VectorFunction, Data)
//
// pub struct CompositeAtom<'a, T: ndarray::Dimension> {
//     points: Vec<&'a Element<'a, T>>
// }

