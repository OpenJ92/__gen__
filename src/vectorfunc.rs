use crate::traits;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct VectorFunc<A, B> {
    pub func: fn(Vec<B>) -> Vec<B>,
    pub callparam: fn(Vec<B>) -> Vec<B>,
    pub export_options: A,
}

impl<A, B> traits::VectorFunction<B> for VectorFunc<A, B> {
    fn call(&self, t: Vec<B>) -> Vec<B> {
        return (self.func)((self.callparam)(t));
    }
}
