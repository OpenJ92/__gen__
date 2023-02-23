use ndarray::{ Array, Ix1 };

#[derive(Clone, Copy, Debug)]
pub enum Atom<'a, T: ndarray::Dimension> {
    Scalar   { scalar: T }, 
    Point    { point:  &'a Array<Ix1, T> }, 
    Line     { start:  &'a Array<Ix1, T>, end: &'a Array<Ix1, T> },
    Triangle { points: [&'a Array<Ix1, T>;3] }, 
}

// Perhaps this is a better formulation 
struct __Point__<'a, T: ndarray::Dimension> {
    point:  &'a Array<Ix1, T> 
}

enum AtomA<'a, T: ndarray::Dimension> {
    Point(__Point__<'a, T>),
    Line(__Point__<'a, T>, __Point__<'a, T>),
    Triangle(__Point__<'a, T>,__Point__<'a, T>, __Point__<'a, T>)
}
// Here we're looking to build a set of vector functions which act upon a heap allocated
// set of Atom(A)s. 
// 
// i.e. Apply(VectorFunction, Data)
