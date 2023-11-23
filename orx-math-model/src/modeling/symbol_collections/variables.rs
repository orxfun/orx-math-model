use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{
            data::{variable::vartype::VariableType, variable_data::VariableData},
            variable::Variable,
        },
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

pub(crate) struct Variables<const D: usize, B: ModelStages>
where
    Variable<D, B>: HasRef<B>,
{
    storage: ImpVec<VariableData<D, B>>,
}
impl<const D: usize, B: ModelStages> Variables<D, B> {
    pub(crate) fn new() -> Self {
        let storage = ImpVec::default();
        Self { storage }
    }
}
impl<const D: usize, B: ModelStages> SymbolCollection for Variables<D, B> {
    type Storage = VariableData<D, B>;
    type Symbol = Variable<D, B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}
impl Variables<0, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.variables.dim0.storage.push(d.build(model));
        }
    }
}
impl Variables<1, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.variables.dim1.storage.push(d.build(model));
        }
    }
}
impl Variables<2, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.variables.dim2.storage.push(d.build(model));
        }
    }
}
impl Variables<3, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.variables.dim3.storage.push(d.build(model));
        }
    }
}
impl Variables<4, Building> {
    fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.variables.dim4.storage.push(d.build(model));
        }
    }
}

pub(crate) struct AllVariables<B: ModelStages> {
    pub(crate) dim0: Variables<0, B>,
    pub(crate) dim1: Variables<1, B>,
    pub(crate) dim2: Variables<2, B>,
    pub(crate) dim3: Variables<3, B>,
    pub(crate) dim4: Variables<4, B>,
}
impl<B: ModelStages> AllVariables<B> {
    pub(crate) fn new() -> Self {
        AllVariables {
            dim0: Variables::<0, _>::new(),
            dim1: Variables::<1, _>::new(),
            dim2: Variables::<2, _>::new(),
            dim3: Variables::<3, _>::new(),
            dim4: Variables::<4, _>::new(),
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
    pub(crate) fn get_key(&self, d: usize, variable_idx: usize) -> &str {
        match d {
            0 => &self.dim0.get(variable_idx).key,
            1 => &self.dim1.get(variable_idx).key,
            2 => &self.dim2.get(variable_idx).key,
            3 => &self.dim3.get(variable_idx).key,
            4 => &self.dim4.get(variable_idx).key,
            _ => panic!("nono"),
        }
    }
    pub(crate) fn get_definition(&self, d: usize, variable_idx: usize) -> &str {
        match d {
            0 => &self.dim0.get(variable_idx).definition,
            1 => &self.dim1.get(variable_idx).definition,
            2 => &self.dim2.get(variable_idx).definition,
            3 => &self.dim3.get(variable_idx).definition,
            4 => &self.dim4.get(variable_idx).definition,
            _ => panic!("nono"),
        }
    }
    pub(crate) fn get_vartype(&self, d: usize, variable_idx: usize) -> VariableType {
        match d {
            0 => self.dim0.get(variable_idx).vartype,
            1 => self.dim1.get(variable_idx).vartype,
            2 => self.dim2.get(variable_idx).vartype,
            3 => self.dim3.get(variable_idx).vartype,
            4 => self.dim4.get(variable_idx).vartype,
            _ => panic!("nono"),
        }
    }
    pub(crate) fn get_bounds_str(&self, d: usize, variable_idx: usize) -> String {
        match d {
            0 => self.dim0.get(variable_idx).bounds.to_string(),
            1 => self.dim1.get(variable_idx).bounds.to_string(),
            2 => self.dim2.get(variable_idx).bounds.to_string(),
            3 => self.dim3.get(variable_idx).bounds.to_string(),
            4 => self.dim4.get(variable_idx).bounds.to_string(),
            _ => panic!("nono"),
        }
    }
}

impl AllVariables<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        self.dim0.add_to_built_model(model);
        self.dim1.add_to_built_model(model);
        self.dim2.add_to_built_model(model);
        self.dim3.add_to_built_model(model);
        self.dim4.add_to_built_model(model);
    }
}
