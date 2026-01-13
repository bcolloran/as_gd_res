use super::expand_as_gd_res;
use super::{assert_eq, parse_quote, quote};

#[test]
fn test_struct_with_generics() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T1 = i32, T2 = String)]
        pub struct StructWithGenerics<T1, T2> {
            pub field1: T1,
            pub field2: T2,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
      impl ::as_gd_res::AsGdRes for StructWithGenerics<i32, String> {
          type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<StructWithGenericsResource>>;
      }

      impl ::as_gd_res::AsGdResOpt for StructWithGenerics<i32, String> {
          type GdOption = Option<::godot::obj::Gd<StructWithGenericsResource>>;
      }

      impl ::as_gd_res::AsGdResArray for StructWithGenerics<i32, String> {
          type GdArray = ::godot::prelude::Array<::godot::obj::Gd<StructWithGenericsResource>>;
      }

      #[derive(::godot::prelude::GodotClass)]
      #[class(tool,init,base=Resource)]
      pub struct StructWithGenericsResource {
          #[base]
          base: ::godot::obj::Base<::godot::classes::Resource>,
          #[export]
          pub field1: <i32 as ::as_gd_res::AsGdRes>::ResType,
          #[export]
          pub field2: <String as ::as_gd_res::AsGdRes>::ResType,
      }

      impl ::as_gd_res::ExtractGd for StructWithGenericsResource {
          type Extracted = StructWithGenerics<i32, String>;
          fn extract(&self) -> Self::Extracted {
              use ::as_gd_res::ExtractGd;
              Self::Extracted {
                  field1: self.field1.extract(),
                  field2: self.field2.extract(),
              }
          }
      }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}
