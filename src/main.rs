use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

trait VectorFunction<T> {
    fn call(&self, t: Vec<T>) -> Vec<T>;
}
trait Exportable {
    fn export(&self) -> String;
}
trait SampleMethod<T> {
    fn values(&self) -> Vec<T>;
}
trait Random<T> {
    fn rand(&self) -> Self;
}

enum Computable<B> {
    Number(B),
    Vector(B),
}

// Consider removing the export options from each of these and placing them into
// a Component type
#[derive(Debug, PartialEq, PartialOrd)]
struct Segment<A, B> {
    p: Vec<B>,
    q: Vec<B>,
    callparam: fn(B) -> B,
    export_options: A,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct PolyLine<A, B> {
    lines: Vec<Vec<B>>,
    callparam: fn(B) -> B,
    export_options: A,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct VectorFunc<A, B> {
    func: fn(Vec<B>) -> Vec<B>,
    callparam: fn(Vec<B>) -> Vec<B>,
    export_options: A,
}

// I really have to consider how we're going to form drawable components.
// Think about how we want to be able to make these drawings.
//
// TakePhotograph { Camera
//                , Scene { [ Sculpture { [ Component { dyn VectorFunctions , dyn SampleTypes, dyn Exportable } ] } ]
//                        , Optional<Lights>
//                        }
//                , Optional<Exportable>
//                }
// where SampleType = Length(Uniform) | Length(Beta) | Curvature etc...
//
// We'd like to have TakePhotograph have a render function

struct SVG {
    fill: String,
    stroke: String,
    stroke_width: f32,
}

struct Processing {
    fill: String,
    stroke: String,
    stroke_width: f32,
}

impl<A, B> VectorFunction<B> for Segment<A, B>
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

impl<A, B> VectorFunction<B> for PolyLine<A, B>
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

impl<A, B> VectorFunction<B> for VectorFunc<A, B> {
    fn call(&self, t: Vec<B>) -> Vec<B> {
        return (self.func)((self.callparam)(t));
    }
}

impl<T> Exportable for Segment<SVG, T>
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

impl<T> Exportable for Segment<Processing, T>
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

impl SVG {
    fn default() -> SVG {
        return SVG {
            fill: String::from("none"),
            stroke: String::from("black"),
            stroke_width: 0.15,
        };
    }
}

impl Processing {
    fn default() -> Processing {
        return Processing {
            fill: String::from("none"),
            stroke: String::from("blue"),
            stroke_width: 0.15,
        };
    }
}

fn main() {
    fn f(x: Vec<f64>) -> Vec<f64> {
        let [x, y, z, w] = match &x[..] {
            [x, y, z, w, ..] => [x, y, z, w],
            _ => panic!(),
        };
        return vec![x * y, z * w];
    }

    let line: Segment<SVG, f64> = Segment {
        p: vec![1.0, 2.0, 3.0, 4.0],
        q: vec![4.0, 3.0, 2.0, 1.0],
        callparam: |t| t,
        export_options: SVG::default(),
    };

    let v0: VectorFunc<SVG, f64> = VectorFunc {
        func: f,
        callparam: |c| c,
        export_options: SVG::default(),
    };

    let v1: VectorFunc<SVG, f64> = VectorFunc {
        func: f,
        callparam: |c| c,
        export_options: SVG::default(),
    };

    VectorFunction::call(&line, vec![1.]);
    VectorFunction::call(&v0, vec![1., 2., 3., 4.]);
    VectorFunction::call(&v1, vec![1., 2., 3., 4.]);

    println!("{}", line.export());

    return ();
}
