use crate::traits;
use crate::segment::Segment;

use num_traits::real::Real;
use num_traits::zero;
use num_traits::one;
// We now introduce new external package in order to specify behavior of 
// parameter B to be "Real"

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PolyLine<B> {
    lines: Vec<Segment<B>>,
    callparam: fn(B) -> B,
}

impl<B> traits::VectorFunction<B> for PolyLine<B>
where
    B: Copy + Real + std::convert::From<usize>
{
    fn call(&self, t: Vec<B>) -> Vec<B> {
        let t: B = match &t[..] {
            [t, ..] => *t,
            _ => panic!(),
        };
        let t: B = (self.callparam)(t);
        let spines : &Vec<Segment<B>> = &self.lines; 
        let length : B = spines.len().into();
        let prime : B = length * t;

        if prime >= length {
            let frac = one(); 
            let whole = length - one();
        } else if prime < zero() {
            let frac = zero();
            let whole = zero();
        } else {
            let frac = prime.fract(); 
            let whole = prime.floor();
        }

        return spines[whole].call(frac)
    }
}
