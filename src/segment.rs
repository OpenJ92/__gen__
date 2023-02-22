use crate::traits;
use ndarray::Array;

use num_traits::real::Real;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Segment<B> {
    pub start: Vec<B>,
    pub end: Vec<B>,
    pub callparam: fn(B) -> B,
}

impl<B> traits::VectorFunction<B> for Segment<B>
where
    B: Real + Copy,
{
    fn call(&self, t: Vec<B>) -> Vec<B> {
        let t: B = match &t[..] {
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
