pub trait VectorFunction<T> {
    fn call(&self, t: Vec<T>) -> Vec<T>;
}
pub trait Exportable {
    fn export_(&self) -> String;
}
pub trait SampleMethod<T> {
    fn values(&self) -> Vec<T>;
}
pub trait Random<T> {
    fn rand(&self) -> Self;
}
