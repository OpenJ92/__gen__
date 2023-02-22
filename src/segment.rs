use crate::traits;
use ndarray::Array;

use num_traits::real::Real;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Segment<B> {
    pub p: Vec<B>,
    pub q: Vec<B>,
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

        let pzq = self.p
            .iter()
            .zip(self.q.iter())
            .map(|u| *u.1 - *u.0);

        return self.p
            .iter()
            .zip(pzq)
            .map(|u| *u.0 + (self.callparam)(t) * u.1)
            .collect();
    }
}

impl<T: Clone> traits::Atomic_ for Segment<T> {}
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
