use crate::traits;
use crate::atomics::{__Point__, __Callable__, Element};
// use ndarray::Array;

use num_traits::real::Real;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Segment<T> {
    pub start: Vec<T>,
    pub end: Vec<T>,
    pub callparam: fn(T) -> T,
}

impl<T> traits::VectorFunction<T> for Segment<T>
where
    T: Real + Copy,
{
    // Like Atom::Point
    fn call(&self, t: Vec<T>) -> Vec<T> {
        let t: T = match &t[..] {
            [t, ..] => *t,
            _ => panic!(),
        };

        let startzend = self.start
            .iter()
            .zip(self.end.iter())
            .map(|u| *u.1 - *u.0);

        return self.start
            .iter()
            .zip(startzend)
            .map(|u| *u.0 + (self.callparam)(t) * u.1)
            .collect();
    }
}

impl<'a, T: ndarray::Dimension> traits::BVectorFunction<'a, T> for Segment<T> {
    fn __call__point__(&self, t: &'a mut __Point__<T>) -> () {
        let __Point__{ point: p } = t;
        p.ndim() == 0;
        
        todo!()
    }
}

// Perhaps this is overkill. Should we have a Atomic Enumeration so we can just 
// pattern match over the possible forms? ie Point, Segment, Triangle, Poly*
//
// #[derive(Clone)]
// pub struct Point<T: Real> {
//     pub point: Vec<T> 
// }
// impl<T: Clone> traits::Atomic_ for Segment<T> {}
// impl<T: Real> traits::Atomic_ for Point<T> {}
// impl<T: Real> traits::Vector_Function for Segment<T> { 
//     fn call<Point<T>>(&self, t) { todo!() }
// }
// impl<T: Clone> traits::Atomic<T> for Segment<T> {}

// impl<T> traits::Exportable for Segment<export::SVG, T>
// where
//     T: std::fmt::Display,
// {
//     fn export_(&self) -> String {
//         return format!(
//             "<line x1=\"{}\" \
//                               x2=\"{}\" \
//                               y1=\"{}\" \
//                               y2=\"{}\" \
//                               fill=\"{}\" \
//                               stroke=\"{}\" \
//                               stroke-width=\"{}\"\
//                         />\n",
//             self.p[0],
//             self.p[1],
//             self.q[0],
//             self.q[1],
//             self.export_options.fill,
//             self.export_options.stroke,
//             self.export_options.stroke_width
//         );
//     }
// }
// 
// impl<T> traits::Exportable for Segment<export::Processing, T>
// where
//     T: std::fmt::Display,
// {
//     fn export_(&self) -> String {
//         return format!(
//             "fill({});\n\
//                         stroke({});\n\
//                         strokeWeight({});\n\
//                         line({}, {}, {}, {});\n",
//             self.export_options.fill,
//             self.export_options.stroke,
//             self.export_options.stroke_width,
//             self.p[0],
//             self.p[1],
//             self.q[0],
//             self.q[1]
//         );
//     }
// }
