#[path = "traits.rs"]
mod traits;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct VectorFunc<A, B> {
    func: fn(Vec<B>) -> Vec<B>,
    callparam: fn(Vec<B>) -> Vec<B>,
    export_options: A,
}

impl<A, B> traits::VectorFunction<B> for VectorFunc<A, B> {
    fn call(&self, t: Vec<B>) -> Vec<B> {
        return (self.func)((self.callparam)(t));
    }
}
