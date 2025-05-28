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

#[derive(Clone, Debug)]
pub struct RustCurve(pub [f32; 100]);

impl_wrapped_as_gd_res!(RustCurve, Curve);

impl ExtractGd for Gd<Curve> {
    type Extracted = RustCurve;

    fn extract(&self) -> Self::Extracted {
        let mut out = [0.0; 100];
        for i in 0..100 {
            out[i] = self.sample(i as f32 / 99.0);
        }
        RustCurve(out)
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
