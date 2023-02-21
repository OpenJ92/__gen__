// use std::ops::Fn;
use wedged::algebra::*;

use ndarray::Array;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

use crate::polyline::PolyLine;

pub mod traits;
use traits::VectorFunction;
pub mod export;
pub mod vectorfunc;
pub mod polyline;
pub mod segment;

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
    fn f(x: Vec<f64>) -> Vec<f64> {
        let [x, y, z, w] = match &x[..] {
            [x, y, z, w, ..] => [x, y, z, w],
            _ => panic!(),
        };
        return vec![x * y, z * w];
    }

    let line: segment::Segment<f64> = segment::Segment {
        p: vec![1.0, 2.0, 3.0, 4.0],
        q: vec![4.0, 3.0, 2.0, 1.0],
        callparam: |t| t,
    };

    let line_: segment::Segment<f64> = segment::Segment {
        p: vec![4.0, 3.0, 2.0, 1.0],
        q: vec![6.0, 2.4, 2.5, 9.0],
        callparam: |t| t,
    };

    let v0: vectorfunc::VectorFunc<f64> = vectorfunc::VectorFunc {
        func: f,
        callparam: |c| c,
    };

    let v1: vectorfunc::VectorFunc<f64> = vectorfunc::VectorFunc {
        func: f,
        callparam: |c| c,
    };

    let pl: polyline::PolyLine<f64> = PolyLine {
        segments: &vec![line, line_], 
        callparam: |c| c,
    };

    VectorFunction::call(&pl, vec![1.]);
    VectorFunction::call(&v0, vec![1., 2., 3., 4.]);
    VectorFunction::call(&v1, vec![1., 2., 3., 4.]);


    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let plane = v1 ^ v2;
    let cross = plane.dual();
    
    assert_eq!(plane, BiVec3::new(0.0,0.0,1.0));
    assert_eq!(cross, Vec3::new(0.0,0.0,1.0));

    let binding = Array::random((3,), Uniform::new(0., 10.));
    let [a, b, c] = match binding.as_slice()
    {
            Some([x, y, z]) => [x, y, z],
            _ => panic!("Input to function must have exactly 3 elements."),
    };
    let t4 = BladeD::from_element(4, 3, 6.28); //dynamic dim, dynamic grade
}

// pub mod atoms {
//     struct Point<A> { vec: Vec<A> }
//     // We should move segment into this module along with polyline.
//     struct Segment { todo!() }
//     struct PolySegment { todo!() }
//     struct Triangle { todo!() } // Ordered list
//     struct PolyTriangle { todo!() }
//     // The VectorFunction trait method should be able to handle all five of these
//     // situations. Be it a computation or a panic!(). They are members of the 
//     // operable class of objects.
// }
//
// in more particular terms, We nust update the VectorFunction trait call to
//
// fn call(t: Atom) -> Atom 
//
// Then, all Atoms will be exportable.


pub mod bezier {
    use crate::traits;
    use ndarray::Array;
    use ndarray::IxDyn;

    struct Bezier<B> {
        pub control_points: Array<IxDyn, B>,
        pub callparam: fn(Vec<B>) -> Vec<B>,
    }

    impl<B> traits::VectorFunction<B> for Bezier<B> {
        fn call(&self, t: Vec<B>) -> Vec<B> {
            todo!();
        }
    }
}

