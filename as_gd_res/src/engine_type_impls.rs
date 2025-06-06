use crate::{AsGdRes, AsGdResArray, AsGdResOpt, ExtractGd};

use crate::impl_wrapped_as_gd_res;
use godot::classes::Curve;
use godot::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedScenePath(pub String);

impl_wrapped_as_gd_res!(PackedScenePath, PackedScene);

impl ExtractGd for Gd<PackedScene> {
    type Extracted = PackedScenePath;
    fn extract(&self) -> Self::Extracted {
        let path = self.get_path();
        PackedScenePath(path.to_string())
    }
}

const CURVE_SAMPLE_POINTS: usize = 64;

#[derive(Clone, Debug)]
pub struct RustCurve {
    baked: [f32; CURVE_SAMPLE_POINTS],
    integral: f32,
    max: f32,
    min: f32,
}

impl RustCurve {
    pub fn sample(&self, x: f32) -> f32 {
        if x < self.min || x > self.max {
            return 0.0;
        }
        let index = ((x - self.min) / (self.max - self.min) * (CURVE_SAMPLE_POINTS as f32 - 1.0))
            .round() as usize;
        self.baked[index]
    }

    pub fn integral(&self) -> f32 {
        self.integral
    }

    pub fn max_domain(&self) -> f32 {
        self.max
    }

    pub fn min_domain(&self) -> f32 {
        self.min
    }
}

impl_wrapped_as_gd_res!(RustCurve, Curve);

impl ExtractGd for Gd<Curve> {
    type Extracted = RustCurve;

    fn extract(&self) -> Self::Extracted {
        let mut baked = [0.0; CURVE_SAMPLE_POINTS];
        let mut integral = 0.0;
        let dx = 1.0 / (CURVE_SAMPLE_POINTS as f32 - 1.0);
        for i in 0..CURVE_SAMPLE_POINTS {
            let y = self.sample(i as f32 / (CURVE_SAMPLE_POINTS as f32 - 1.0));
            baked[i] = y;
            integral += y * dx;
        }
        RustCurve {
            baked,
            integral,
            max: self.get_max_domain(),
            min: self.get_min_domain(),
        }
    }
}

impl AsGdRes for String {
    type ResType = GString;
}
impl ExtractGd for GString {
    type Extracted = String;
    fn extract(&self) -> Self::Extracted {
        self.to_string()
    }
}
