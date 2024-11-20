use crate::modeling::symbols::data::set::{
    data_and_fun_dim1::DataAndFunDim1,
    set_elements::{Elements, SetElements},
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    rc::Rc,
};

// data and fun
pub trait IntoDepDataAndFunDim1<Data> {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements;
}

impl<Data, Fun> IntoDepDataAndFunDim1<Data> for (Rc<Data>, Fun)
where
    Rc<Data>: 'static,
    Fun: for<'a> Fn(&'a Data, usize) -> Elements<'a> + 'static,
{
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements {
        let data_and_fun = DataAndFunDim1 {
            data: self.0,
            fun: Rc::new(self.1),
        };
        SetElements::Dep1DataAndFun(dependent_set_idx, Rc::new(data_and_fun))
    }
}

// data
pub trait IntoDepDataDim1 {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements;
}

impl IntoDepDataDim1 for Rc<Vec<Vec<usize>>> {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements {
        SetElements::Dep1VecVec(dependent_set_idx, self)
    }
}
impl IntoDepDataDim1 for Rc<Vec<HashSet<usize>>> {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements {
        SetElements::Dep1VecSet(dependent_set_idx, self)
    }
}
impl IntoDepDataDim1 for Rc<HashMap<usize, Vec<usize>>> {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements {
        SetElements::Dep1HashMapVec(dependent_set_idx, self)
    }
}
impl IntoDepDataDim1 for Rc<HashMap<usize, HashSet<usize>>> {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements {
        SetElements::Dep1HashMapSet(dependent_set_idx, self)
    }
}
impl IntoDepDataDim1 for Rc<BTreeMap<usize, Vec<usize>>> {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements {
        SetElements::Dep1BTreeMapVec(dependent_set_idx, self)
    }
}
impl IntoDepDataDim1 for Rc<BTreeMap<usize, HashSet<usize>>> {
    fn into_set_elements(self, dependent_set_idx: usize) -> SetElements {
        SetElements::Dep1BTreeMapSet(dependent_set_idx, self)
    }
}
