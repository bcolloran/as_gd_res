use std::str::FromStr;

use crate::{AsGdRes, AsGdResArray, AsGdResOpt, ExtractGd, impl_wrapped_builtin_as_gd_res};

use crate::impl_wrapped_as_gd_res;
use godot::classes::Curve;
use godot::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "easy_hash", derive(easy_hash::EasyHash))]
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
#[cfg_attr(feature = "easy_hash", derive(easy_hash::EasyHash))]
pub struct RustCurve {
    baked: [f32; CURVE_SAMPLE_POINTS],
    integral: f32,
    max: f32,
    min: f32,
}

impl RustCurve {
    pub fn try_sample(&self, x: f32) -> Result<f32, String> {
        if x < self.min {
            return Err(format!(
                "Value {} is below the minimum domain {}",
                x, self.min
            ));
        }
        if x > self.max {
            return Err(format!(
                "Value {} is above the maximum domain {}",
                x, self.max
            ));
        }
        // Handle zero-width domain (min == max) as a special case
        // to avoid division by zero which would produce NaN
        if (self.max - self.min).abs() < f32::EPSILON {
            // For a zero-width domain, return the first baked value
            return Ok(self.baked[0]);
        }
        let index = ((x - self.min) / (self.max - self.min) * (CURVE_SAMPLE_POINTS as f32 - 1.0))
            .round() as usize;
        Ok(self.baked[index])
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

    /// Creates a new RustCurve for testing purposes.
    /// This constructor is only available in test builds.
    #[cfg(test)]
    pub fn new_for_test(baked: [f32; CURVE_SAMPLE_POINTS], integral: f32, min: f32, max: f32) -> Self {
        Self {
            baked,
            integral,
            max,
            min,
        }
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

impl_wrapped_builtin_as_gd_res!(String, GString);

impl ExtractGd for GString {
    type Extracted = String;
    fn extract(&self) -> Self::Extracted {
        self.to_string()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "easy_hash", derive(easy_hash::EasyHash))]
pub struct NodePathString(pub String);
impl_wrapped_builtin_as_gd_res!(NodePathString, NodePath);

impl NodePathString {
    pub fn to_node_path(&self) -> NodePath {
        NodePath::from_str(&self.0).unwrap()
        // self.0.clone().to_string()
    }
}

impl ExtractGd for NodePath {
    type Extracted = NodePathString;
    fn extract(&self) -> Self::Extracted {
        NodePathString(self.to_string())
    }
}
