use ndarray::{ Array, Ix1 };

#[derive(Clone, Copy, Debug)]
pub enum Atom<'a, T: ndarray::Dimension> {
    Scalar   { scalar: T }, 
    Point    { point:  &'a Array<Ix1, T> }, 
    Line     { start:  &'a Array<Ix1, T>, end: &'a Array<Ix1, T> },
    Triangle { points: [&'a Array<Ix1, T>;3] }, 
}

// Perhaps this is a better formulation 
pub struct __Point__<'a, T: ndarray::Dimension> {
    point:  &'a Array<Ix1, T> 
}

pub enum AtomA<'a, T: ndarray::Dimension> {
    Point(&'a __Point__<'a, T>),
    Line(&'a __Point__<'a, T>, &'a __Point__<'a, T>),
    Triangle(&'a __Point__<'a, T>,&'a __Point__<'a, T>, &'a __Point__<'a, T>)
}

pub struct CompositeAtom<'a, T: ndarray::Dimension> {
    points: Vec<&'a AtomA<'a, T>>
}

pub enum __Callable__<'a, T: ndarray::Dimension> {
    Atomic(&'a AtomA<'a, T>),
    Composite(&'a CompositeAtom<'a, T>)
}
// Here we're looking to build a set of vector functions which act upon a heap allocated
// set of Atom(A)s. 
// 
// i.e. Apply(VectorFunction, Data)
