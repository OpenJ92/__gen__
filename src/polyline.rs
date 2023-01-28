#[path = "traits.rs"]
mod traits;


#[derive(Debug, PartialEq, PartialOrd)]
pub struct PolyLine<A, B> {
    lines: Vec<Vec<B>>,
    callparam: fn(B) -> B,
    export_options: A,
}

impl<A, B> traits::VectorFunction<B> for PolyLine<A, B>
where
    B: Copy,
{
    fn call(&self, t: Vec<B>) -> Vec<B> {
        let t: B = match &t[..] {
            [t, ..] => *t,
            _ => panic!(),
        };
        // let t: Vec<B> = self.callparam(t);
        // let spines = self.many(); length = spines.len();
        todo!();
    }
}
