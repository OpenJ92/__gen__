use crate::traits;
use crate::segment::Segment;


use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use num_traits::real::Real;
use num_traits::zero;
use num_traits::one;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PolyLine<'a, B> {
    pub segments: &'a Vec<Segment<B>>,
    pub callparam: fn(B) -> B,
}

impl<'a, B> traits::VectorFunction<B> for PolyLine<'a, B>
where
    B: Copy + Real + FromPrimitive + ToPrimitive, 
{
    fn call(&self, t: Vec<B>) -> Vec<B> {
        // Capture Real number to apply to VectorFunction::Polyline
        let t: B = match &t[..] {
            [t, ..] => *t,
            _ => panic!(),
        };

        // Update Real number according to paraemterization. Query length of
        // segments. Thier product whole.frac will be the vector accessor, whole, 
        // and call, frac, to the appropriate Segment inside.
        let t        : B     = (self.callparam)(t);
        let length   : usize = self.segments.len();
        let location : B     = t * B::from_usize(length).unwrap();

        // Check location to be interior to the size of the vector.
        let (whole, frac) = 
            if location >= B::from_usize(length).unwrap() {
                let frac  : B     = one(); 
                let whole : usize = length - one::<usize>();
                (whole, frac)
            } else if location < zero() {
                let frac  : B     = zero();
                let whole : usize = zero();
                (whole, frac)
            } else {
                let frac  : B     = location.fract(); 
                let whole : usize = location.floor().to_usize().unwrap();
                (whole, frac)
            };

        // Capture the appropriate segment from segments and call on frac.
        return self.segments[whole].call(vec!(frac))
    }
}
