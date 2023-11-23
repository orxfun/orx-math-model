use super::definition::ParameterDefinition;
use crate::data_structs::funvec::{DataAndFunVec, FunVec};
use crate::modeling::symbol_collections::parameters::Parameters;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::numerics::num::RealNum;
use crate::{
    modeling::stages::Building,
    modeling::{
        modeler::SymbolFromBuilder,
        reference::HasRef,
        symbols::{data::parameter_data::ParameterData, parameter::Parameter},
    },
};
use std::rc::Rc;

pub struct ParameterDefinitionWithDimension<const D: usize> {
    pub(crate) definition: ParameterDefinition,
}

impl<const D: usize> ParameterDefinitionWithDimension<D> {
    fn push_get_param(
        self,
        params: &Parameters<D, Building>,
        fun: Rc<dyn FunVec<D, RealNum>>,
    ) -> Parameter<D, Building> {
        let sym_ref = self
            .definition
            .modeler
            .new_index(SymbolFromBuilder::Parameter { dim: D });
        let (key, definition) = self.definition.symbol.into_key_definition();
        let symbol = Parameter::new(sym_ref);
        let storage = ParameterData {
            symbol,
            fun,
            key,
            definition,
        };
        params.push(storage)
    }
}

// dims
impl ParameterDefinitionWithDimension<0> {
    pub fn from_data<Data>(self, data: Data) -> Parameter<0, Building>
    where
        Data: FunVec<0, RealNum> + 'static,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim0;
        let fun = Rc::new(data);
        self.push_get_param(params, fun)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Parameter<0, Building>
    where
        Rc<Data>: 'static,
        (Rc<Data>, Fun): Into<DataAndFunVec<0, Data, RealNum>>,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim0;
        let data_and_fun = (data, fun).into();
        let fun = Rc::new(data_and_fun);
        self.push_get_param(params, fun)
    }
}
impl ParameterDefinitionWithDimension<1> {
    pub fn from_data<Data>(self, data: Data) -> Parameter<1, Building>
    where
        Data: FunVec<1, RealNum> + 'static,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim1;
        let fun = Rc::new(data);
        self.push_get_param(params, fun)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Parameter<1, Building>
    where
        Rc<Data>: 'static,
        (Rc<Data>, Fun): Into<DataAndFunVec<1, Data, RealNum>>,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim1;
        let data_and_fun = (data, fun).into();
        let fun = Rc::new(data_and_fun);
        self.push_get_param(params, fun)
    }
}
impl ParameterDefinitionWithDimension<2> {
    pub fn from_data<Data>(self, data: Data) -> Parameter<2, Building>
    where
        Data: FunVec<2, RealNum> + 'static,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim2;
        let fun = Rc::new(data);
        self.push_get_param(params, fun)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Parameter<2, Building>
    where
        Rc<Data>: 'static,
        (Rc<Data>, Fun): Into<DataAndFunVec<2, Data, RealNum>>,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim2;
        let data_and_fun = (data, fun).into();
        let fun = Rc::new(data_and_fun);
        self.push_get_param(params, fun)
    }
}
impl ParameterDefinitionWithDimension<3> {
    pub fn from_data<Data>(self, data: Data) -> Parameter<3, Building>
    where
        Data: FunVec<3, RealNum> + 'static,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim3;
        let fun = Rc::new(data);
        self.push_get_param(params, fun)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Parameter<3, Building>
    where
        Rc<Data>: 'static,
        (Rc<Data>, Fun): Into<DataAndFunVec<3, Data, RealNum>>,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim3;
        let data_and_fun = (data, fun).into();
        let fun = Rc::new(data_and_fun);
        self.push_get_param(params, fun)
    }
}
impl ParameterDefinitionWithDimension<4> {
    pub fn from_data<Data>(self, data: Data) -> Parameter<4, Building>
    where
        Data: FunVec<4, RealNum> + 'static,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim4;
        let fun = Rc::new(data);
        self.push_get_param(params, fun)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Parameter<4, Building>
    where
        Rc<Data>: 'static,
        (Rc<Data>, Fun): Into<DataAndFunVec<4, Data, RealNum>>,
    {
        let modeler = self.definition.modeler.clone();
        let params = &modeler.data.parameters.dim4;
        let data_and_fun = (data, fun).into();
        let fun = Rc::new(data_and_fun);
        self.push_get_param(params, fun)
    }
}
