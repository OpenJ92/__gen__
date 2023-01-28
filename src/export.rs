pub mod export;

pub struct SVG {
    fill: String,
    stroke: String,
    stroke_width: f32,
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

pub struct Processing {
    fill: String,
    stroke: String,
    stroke_width: f32,
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
