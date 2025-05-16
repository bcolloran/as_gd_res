use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

use super::derive_inner;

#[test]
fn test_1() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum BrainState {
            DirectChaser,
            Roomba,
            Tank(EnumInnerState),
        }
    };

    let expected = quote! {
    impl ExtractGd for BrainState
    where BrainState : Clone
    {
        type Extracted = BrainState;
        fn extract(&self) -> Self::Extracted {
            self.clone()
        }
    }
    };

    assert_eq!(derive_inner(input).to_string(), expected.to_string());
}

#[test]
fn test_2() {
    let input: syn::DeriveInput = parse_quote! {
        #[extract_to=EnemyParamsExtracted]
        pub struct EnemyParams {
            #[base]
            base: Base<Resource>,

            #[export]
            pub brain_params_required: OnEditor<DynGd<Resource, dyn BrainParamsDynRes>>,

            #[export]
            pub brain_params_optional: Option<DynGd<Resource, dyn BrainParamsDynRes>>,

            #[export]
            pub drop_params: Option<Gd<DropParams>>,
        }
    };

    let expected = quote! {
    pub struct EnemyParamsExtracted {
        pub brain_params_required:
            <OnEditor<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
        pub brain_params_optional:
            <Option<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
        pub drop_params: <Option<Gd<DropParams>> as ExtractGd>::Extracted,
    }

    /// WE WANT TO DEFINE THIS IMPL VIA MACRO!
    impl ExtractGd for EnemyParams {
        type Extracted = EnemyParamsExtracted;
        fn extract(&self) -> Self::Extracted {
            Self::Extracted {
                brain_params_required: self.brain_params_required.extract(),
                brain_params_optional: self.brain_params_optional.extract(),
                drop_params: self.drop_params.extract(),
            }
        }
    }
        };

    assert_eq!(derive_inner(input).to_string(), expected.to_string());
}
