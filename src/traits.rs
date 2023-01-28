pub mod traits;

pub trait VectorFunction<T> {
    pub fn call(&self, t: Vec<T>) -> Vec<T>;
}
pub trait Exportable {
    pub fn export(&self) -> String;
}
pub trait SampleMethod<T> {
    pub fn values(&self) -> Vec<T>;
}
pub trait Random<T> {
    pub fn rand(&self) -> Self;
}
