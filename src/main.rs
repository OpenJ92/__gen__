// use std::ops::Fn;
use wedged::algebra::*;
use ndarray::Array;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

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

    let line: Segment<export::SVG, f64> = Segment {
        p: vec![1.0, 2.0, 3.0, 4.0],
        q: vec![4.0, 3.0, 2.0, 1.0],
        callparam: |t| t,
        export_options: export::SVG::default(),
    };

    let v0: vectorfunc::VectorFunc<export::SVG, f64> = vectorfunc::VectorFunc {
        func: f,
        callparam: |c| c,
        export_options: export::SVG::default(),
    };

    let v1: vectorfunc::VectorFunc<export::SVG, f64> = vectorfunc::VectorFunc {
        func: f,
        callparam: |c| c,
        export_options: export::SVG::default(),
    };

    line.call(vec![1.]);
    // traits::VectorFunction::call(&v0, vec![1., 2., 3., 4.]);
    // traits::VectorFunction::call(&v1, vec![1., 2., 3., 4.]);
    

    // println!("{}", line._export());

    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let plane = v1 ^ v2;
    let cross = plane.dual();
    
    assert_eq!(plane, BiVec3::new(0.0,0.0,1.0));
    assert_eq!(cross, Vec3::new(0.0,0.0,1.0));

    let binding = Array::random((5,), Uniform::new(0., 10.));
    let [a, b, c] = match binding.as_slice()
    {
            Some([x, y, z]) => [x, y, z],
            _ => panic!("Input to function must have exactly 3 elements."),
    };
    let t4 = BladeD::from_element(4, 3, 6.28); //dynamic dim, dynamic grade
    println!("{:8.4}", t4);

    let q = Vec3::new(a, b, c);
    // let r = std::ops::Fn::call(&Vec3::new, binding);
}
