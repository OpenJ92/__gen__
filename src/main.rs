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

    let line: segment::Segment<export::SVG, f64> = segment::Segment {
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

    traits::VectorFunction::call(&line, vec![1.]);
    traits::VectorFunction::call(&v0, vec![1., 2., 3., 4.]);
    traits::VectorFunction::call(&v1, vec![1., 2., 3., 4.]);
    

    // println!("{}", line._export());

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

    let q = Vec3::new(a, b, c);
    // let r = std::ops::Fn::call(&Vec3::new, binding);
}

pub mod traits {
    pub trait VectorFunction<T> {
        fn call(&self, t: Vec<T>) -> Vec<T>;
    }
    pub trait Exportable {
        fn export_(&self) -> String;
    }
    pub trait SampleMethod<T> {
        fn values(&self) -> Vec<T>;
    }
    pub trait Random<T> {
        fn rand(&self) -> Self;
    }
}

pub mod export {
    pub struct SVG {
        pub fill: String,
        pub stroke: String,
        pub stroke_width: f32,
    }
    
    impl SVG {
        pub fn default() -> SVG {
            return SVG {
                fill: String::from("none"),
                stroke: String::from("black"),
                stroke_width: 0.15,
            };
        }
    }
    
    pub struct Processing {
        pub fill: String,
        pub stroke: String,
        pub stroke_width: f32,
    }
    
    impl Processing {
        pub fn default() -> Processing {
            return Processing {
                fill: String::from("none"),
                stroke: String::from("blue"),
                stroke_width: 0.15,
            };
        }
    }
}

pub mod vectorfunc {
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
}

pub mod polyline {
    use crate::traits;

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
}

pub mod segment {
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::Sub;
    
    use crate::traits;
    use crate::export;
    
    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct Segment<A, B> {
        pub p: Vec<B>,
        pub q: Vec<B>,
        pub callparam: fn(B) -> B,
        pub export_options: A,
    }
    
    impl<A, B> traits::VectorFunction<B> for Segment<A, B>
    where
        B: Add<Output = B> + Mul<Output = B> + Sub<Output = B> + Copy,
    {
        fn call(&self, t: Vec<B>) -> Vec<B> {
            let t: B = match &t[..] {
                [t, ..] => *t,
                _ => panic!(),
            };
    
            let pzq = self.p.iter().zip(self.q.iter());
            let pzqm = pzq.map(|u| *u.1 - *u.0);
    
            let pzpzqm = self.p.iter().zip(pzqm);
            let result = pzpzqm.map(|u| *u.0 + (self.callparam)(t) * u.1);
    
            return result.collect();
        }
    }
    
    impl<T> traits::Exportable for Segment<export::SVG, T>
    where
        T: std::fmt::Display,
    {
        fn export_(&self) -> String {
            return format!(
                "<line x1=\"{}\"\
                                  x2=\"{}\"\
                                  y1=\"{}\"\
                                  y2=\"{}\"\
                                  fill=\"{}\"\
                                  stroke=\"{}\"\
                                  stroke-width=\"{}\"\
                            />\n",
                self.p[0],
                self.p[1],
                self.q[0],
                self.q[1],
                self.export_options.fill,
                self.export_options.stroke,
                self.export_options.stroke_width
            );
        }
    }
    
    impl<T> traits::Exportable for Segment<export::Processing, T>
    where
        T: std::fmt::Display,
    {
        fn export_(&self) -> String {
            return format!(
                "fill({});\n\
                            stroke({});\n\
                            strokeWeight({});\n\
                            line({}, {}, {}, {});\n",
                self.export_options.fill,
                self.export_options.stroke,
                self.export_options.stroke_width,
                self.p[0],
                self.p[1],
                self.q[0],
                self.q[1]
            );
        }
    }
}
