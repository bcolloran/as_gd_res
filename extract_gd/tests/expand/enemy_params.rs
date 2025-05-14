#[derive(ExtractGd)]
#[extract_to(EnemyParamsExtracted)]
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
