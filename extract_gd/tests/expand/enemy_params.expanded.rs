pub struct EnemyParamsExtracted {
    pub brain_params_required:
        <OnEditor<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
    pub brain_params_optional:
        <Option<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
    pub drop_params: <Option<Gd<DropParams>> as ExtractGd>::Extracted,
}
impl ExtractGd for EnemyParams {
    type Extracted = EnemyParamsExtracted;
    fn extract(&self) -> Self::Extracted {
        EnemyParamsExtracted {
            brain_params_required: self.brain_params_required.extract(),
            brain_params_optional: self.brain_params_optional.extract(),
            drop_params: self.drop_params.extract(),
        }
    }
}
