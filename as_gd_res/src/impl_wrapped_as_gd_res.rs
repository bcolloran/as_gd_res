#[macro_export]
macro_rules! impl_wrapped_as_gd_res {
    ($t_rust:ty, $t_gd:ty) => {
        impl AsGdRes for $t_rust {
            type ResType = ::godot::prelude::OnEditor<Gd<$t_gd>>;
        }

        impl AsGdResOpt for $t_rust {
            type GdOption = Option<Gd<$t_gd>>;
        }

        impl AsGdResArray for $t_rust {
            type GdArray = Array<Gd<$t_gd>>;
        }
    };
    () => {};
}
