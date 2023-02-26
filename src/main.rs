// use std::ops::Fn;
// use wedged::algebra::*;
// 
// use ndarray::Array;
// use ndarray_rand::RandomExt;
// use ndarray_rand::rand_distr::Uniform;

use crate::polyline::PolyLine;

use ndarray::{ array, Array, Ix1 };
use ndarray_rand::rand_distr::Normal;


pub mod traits;
pub mod export;
pub mod vectorfunc;
pub mod polyline;
pub mod segment;
pub mod atomics;

// use crate::segment::Segment;
// use crate::vectorfunc::VectorFunc;

// Consider removing the export options from each of these and placing them into
// a Component type

// I really have to consider how we're going to form drawable components.
// Think about how we want to be able to make these drawings.
//
// TakePhotograph { Camera
//                , Scene {[ Sculpture { [ Component { dyn VectorFunctions , dyn SampleTypes, dyn Exportable } ] } ]
//                        , Optional<Lights>
//                        }
//                , Optional<Exportable>
//                }
// where SampleType = Length(Uniform) | Length(Beta) | Curvature etc...
//
// We'd like to have TakePhotograph have a render function
fn main() { 
    let s = segment::Segment::<f32> {
        start: array!(1.,1.,1.,1.),
        end:   array!(1.,1.,2.,2.),
        callparam: |t| t
    };

    let mut arr = array!(1.,1.);
    println!("{:?}", arr);

    let mut binding = atomics::Element::Point(&mut arr);
    let mut t = atomics::__Callable__::Atomic(&mut binding);
    traits::VectorFunction::__call_dispatch__(&s, &mut t);
    println!("{:?}", arr);
}


pub mod bezier {
    use crate::atomics::__Callable__;
    use ndarray::Array;
    use ndarray::IxDyn;

    struct Bezier<'a, B> {
        pub control_points: Array<IxDyn, B>,
        pub callparam: fn(__Callable__<'a, B>) -> __Callable__<'a, B>,
    }
}

