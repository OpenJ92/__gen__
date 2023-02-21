pub trait SampleMethod<T> {
    fn values(&self) -> Vec<T>;
}
pub trait VectorFunction<T> {
    fn call(&self, t: Vec<T>) -> Vec<T>;
    fn sample(&self, sample_method: impl SampleMethod<T>) -> Vec<Vec<T>> {
        return sample_method.values().iter().map(|t| self.call(vec!(*t))).collect()
    }
}
pub trait Exportable { 
    fn export_(&self) -> String;
}
pub trait Random<T> {
    fn rand(&self) -> Self;
}
