#[path = "traits.rs"]
mod traits;
#[path = "export.rs"]
mod export;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Segment<A, B> {
    p: Vec<B>,
    q: Vec<B>,
    callparam: fn(B) -> B,
    export_options: A,
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
    fn export(&self) -> String {
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
    fn export(&self) -> String {
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
