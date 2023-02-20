use crate::traits;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct VectorFunc<B> {
    pub func: fn(Vec<B>) -> Vec<B>,
    pub callparam: fn(Vec<B>) -> Vec<B>,
}

impl<B> traits::VectorFunction<B> for VectorFunc<B> {
    fn call(&self, t: Vec<B>) -> Vec<B> {
        return (self.func)((self.callparam)(t));
    }
}
