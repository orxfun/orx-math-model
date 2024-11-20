use crate::modeling::symbols::data::set::{
    data_and_fun_dim0::DataAndFunDim0,
    set_elements::{Elements, SetElements},
};
use std::{collections::HashSet, rc::Rc};

// data and fun
pub trait IntoSetDataAndDataIndependent<Data> {
    fn into_set_elements(self) -> SetElements;
}

impl<Data, Fun> IntoSetDataAndDataIndependent<Data> for (Rc<Data>, Fun)
where
    Rc<Data>: 'static,
    Fun: for<'a> Fn(&'a Data) -> Elements<'a> + 'static,
{
    fn into_set_elements(self) -> SetElements {
        let data_and_fun = DataAndFunDim0 {
            data: self.0,
            fun: Rc::new(self.1),
        };
        SetElements::DataAndFun(Rc::new(data_and_fun))
    }
}

// data
pub trait IntoSetDataIndependent {
    fn into_set_elements(self) -> SetElements;
}

impl IntoSetDataIndependent for Rc<Vec<usize>> {
    fn into_set_elements(self) -> SetElements {
        SetElements::Vec(self)
    }
}
impl IntoSetDataIndependent for Rc<HashSet<usize>> {
    fn into_set_elements(self) -> SetElements {
        SetElements::Set(self)
    }
}
