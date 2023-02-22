use ndarray::{ Array, Ix1 };

#[derive(Clone)]
pub enum Atom<T> {
    Point    { point:  Array<Ix1, T> }, 
    Line     { start:  Array<Ix1, T>, end: Array<Ix1, T> },
    Triangle { points: [Array<Ix1, T>;3] }, 
}
