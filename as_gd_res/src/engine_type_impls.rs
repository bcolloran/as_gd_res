use crate::{AsGdRes, AsGdResArray, AsGdResOpt, ExtractGd};

use crate::impl_wrapped_as_gd_res;
use godot::classes::Curve;
use godot::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedScenePath(pub String);

impl_wrapped_as_gd_res!(PackedScenePath, PackedScene);

// impl AsGdRes for PackedScenePath {
//     type ResType = OnEditor<Gd<PackedScene>>;
// }
// impl AsGdRes for Option<PackedScenePath> {
//     type ResType = Option<Gd<PackedScene>>;
// }
// impl AsGdRes for Vec<PackedScenePath> {
//     type ResType = Array<Gd<PackedScene>>;
// }

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

// impl AsGdRes for RustCurve {
//     type ResType = OnEditor<Gd<Curve>>;
// }

// impl AsGdResOpt for RustCurve {
//     type GdTypeOpt = Option<Gd<Curve>>;
// }

// impl AsGdResArray for RustCurve {
//     type GdArray = Array<Gd<Curve>>;
// }

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
