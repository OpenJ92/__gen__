use ndarray::{ Array, Ix1 };

pub enum Element<'a, T> {
    Point(&'a mut Array<T, Ix1>),
    Line(&'a mut Array<T, Ix1>, &'a mut Array<T, Ix1>),
    Triangle(&'a mut Array<T, Ix1>, &'a mut Array<T, Ix1>, &'a mut Array<T, Ix1>)
}
pub enum __Callable__<'a, T> {
    Atomic(&'a mut Element<'a, T>),
    Composite(Vec<&'a mut __Callable__<'a, T>>)
}
