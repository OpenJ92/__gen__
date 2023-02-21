pub trait Atomic<T> : Copy {}
pub trait SampleMethod<T> {
    // As we continue to develop this software, this form should become
    // fn values(&self) -> Vec<impl Atomic>. Atomic should be exportable
    // forms and callable forms. With that, the VectorFunction required
    // function call should be fn call(&self, t: impl Atomic> -> impl Atomic.
    fn values(&self) -> Vec<Vec<T>>;
}
pub trait VectorFunction<T> {
    // fn call(&self, t: impl Atomic<T>) -> impl Atomic<T>;
    fn call(&self, t: Vec<T>) -> Vec<T>;
    fn sample(&self, sample_method: impl SampleMethod<T>) -> Vec<Vec<T>> {
        return sample_method.values().iter().map(move |t| self.call(*t)).collect()
    }
}
pub trait Exportable { 
    fn export_(&self) -> String;
}
pub trait Random<T> {
    fn rand(&self) -> Self;
}
