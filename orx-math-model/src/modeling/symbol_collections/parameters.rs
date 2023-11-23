use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{data::parameter_data::ParameterData, parameter::Parameter},
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Parameters<const D: usize, B: ModelStages>
where
    Parameter<D, B>: HasRef<B>,
{
    storage: ImpVec<ParameterData<D, B>>,
}
impl<const D: usize, B: ModelStages> Parameters<D, B> {
    pub(crate) fn new() -> Self {
        let storage = ImpVec::default();
        Self { storage }
    }
}
impl<const D: usize, B: ModelStages> SymbolCollection for Parameters<D, B> {
    type Storage = ParameterData<D, B>;
    type Symbol = Parameter<D, B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}
impl Parameters<0, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.parameters.dim0.storage.push(d.build(model));
        }
    }
}
impl Parameters<1, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.parameters.dim1.storage.push(d.build(model));
        }
    }
}
impl Parameters<2, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.parameters.dim2.storage.push(d.build(model));
        }
    }
}
impl Parameters<3, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.parameters.dim3.storage.push(d.build(model));
        }
    }
}
impl Parameters<4, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.parameters.dim4.storage.push(d.build(model));
        }
    }
}

#[derive(Default)]
pub(crate) struct AllParameters<B: ModelStages> {
    pub(crate) dim0: Parameters<0, B>,
    pub(crate) dim1: Parameters<1, B>,
    pub(crate) dim2: Parameters<2, B>,
    pub(crate) dim3: Parameters<3, B>,
    pub(crate) dim4: Parameters<4, B>,
}
impl<B: ModelStages> AllParameters<B> {
    pub(crate) fn new() -> Self {
        AllParameters {
            dim0: Parameters::<0, _>::new(),
            dim1: Parameters::<1, _>::new(),
            dim2: Parameters::<2, _>::new(),
            dim3: Parameters::<3, _>::new(),
            dim4: Parameters::<4, _>::new(),
        }
    }
    pub(crate) fn iter_debug(&self) -> impl Iterator<Item = String> + '_ {
        self.dim0
            .storage
            .iter()
            .map(|x| format!("{:?}", x.symbol))
            .chain(self.dim1.storage.iter().map(|x| format!("{:?}", x.symbol)))
            .chain(self.dim2.storage.iter().map(|x| format!("{:?}", x.symbol)))
            .chain(self.dim3.storage.iter().map(|x| format!("{:?}", x.symbol)))
            .chain(self.dim4.storage.iter().map(|x| format!("{:?}", x.symbol)))
    }
    pub(crate) fn all_model_references(&self) -> impl Iterator<Item = &SymRef<B>> {
        self.dim0
            .storage
            .iter()
            .map(|x| x.symbol.sym_ref())
            .chain(self.dim1.storage.iter().map(|x| x.symbol.sym_ref()))
            .chain(self.dim2.storage.iter().map(|x| x.symbol.sym_ref()))
            .chain(self.dim3.storage.iter().map(|x| x.symbol.sym_ref()))
            .chain(self.dim4.storage.iter().map(|x| x.symbol.sym_ref()))
    }
    pub(crate) fn get_key(&self, d: usize, par_idx: usize) -> &str {
        match d {
            0 => &self.dim0.get(par_idx).key,
            1 => &self.dim1.get(par_idx).key,
            2 => &self.dim2.get(par_idx).key,
            3 => &self.dim3.get(par_idx).key,
            4 => &self.dim4.get(par_idx).key,
            _ => panic!("nono"),
        }
    }
    pub(crate) fn get_definition(&self, d: usize, par_idx: usize) -> &str {
        match d {
            0 => &self.dim0.get(par_idx).definition,
            1 => &self.dim1.get(par_idx).definition,
            2 => &self.dim2.get(par_idx).definition,
            3 => &self.dim3.get(par_idx).definition,
            4 => &self.dim4.get(par_idx).definition,
            _ => panic!("nono"),
        }
    }
}

impl AllParameters<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        self.dim0.add_to_built_model(model);
        self.dim1.add_to_built_model(model);
        self.dim2.add_to_built_model(model);
        self.dim3.add_to_built_model(model);
        self.dim4.add_to_built_model(model);
    }
}
