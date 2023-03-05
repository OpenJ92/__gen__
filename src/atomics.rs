use ndarray::{ Array, IxDyn };

#[derive(Debug,)]
pub enum Element<T> {
    Point(Array<T, IxDyn>),
    Line(Array<T, IxDyn>, Array<T, IxDyn>),
    Triangle(Array<T, IxDyn>, Array<T, IxDyn>, Array<T, IxDyn>)
}

#[derive(Debug,)]
pub enum __Callable__<T> {
    Atomic(Element<T>),
    Composite(Vec<__Callable__<T>>)
}
