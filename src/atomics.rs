use ndarray::{ Array, Ix1 };

#[derive(Clone, Copy)]
pub enum Atom<'a, T> {
    Scalar   { scalar: T }, 
    Point    { point:  &'a Array<Ix1, T> }, 
    Line     { start:  &'a Array<Ix1, T>, end: &'a Array<Ix1, T> },
    Triangle { points: [&'a Array<Ix1, T>;3] }, 
}

// Here we're looking to build a set of vector functions which act upon a heap allocated
// set of Atoms. 
// 
// i.e. Apply(VectorFunction, Data)
