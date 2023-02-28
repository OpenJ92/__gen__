use ndarray::{ Array, IxDyn };

pub enum Element<'a, T> {
    Point(&'a mut Array< T, IxDyn>),
    Line(&'a mut Array< T, IxDyn>, &'a mut Array< T, IxDyn>),
    Triangle(&'a mut Array< T, IxDyn>, &'a mut Array< T, IxDyn>, &'a mut Array< T, IxDyn>)
}
pub enum __Callable__<'a, T> {
    Atomic(&'a mut Element<'a, T>),
    Composite(Vec<&'a mut __Callable__<'a, T>>)
}
