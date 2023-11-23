use super::{
    bounded_by::BoundedSet, data_and_fun_dim0::DataAndFunSet0, data_and_fun_dim1::DataAndFunSet1,
    next_prev_of::NextPrevBound,
};
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::symbols::data::set::{
        dep1_data::{
            dep1_btreemap_set, dep1_btreemap_vec, dep1_hashmap_set, dep1_hashmap_vec, dep1_vec_set,
            dep1_vec_vec,
        },
        other_than::elements_other_than,
    },
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::Range,
    rc::Rc,
};

pub(crate) type Elements<'a> = Box<dyn Iterator<Item = usize> + 'a>;

#[derive(Clone)]
pub enum SetElements {
    // independent
    Range(Range<usize>),
    Vec(Rc<Vec<usize>>),
    Set(Rc<HashSet<usize>>),
    DataAndFun(Rc<dyn DataAndFunSet0>),

    // dependent-dim1
    Dep1DataAndFun(usize, Rc<dyn DataAndFunSet1>),
    Dep1VecVec(usize, Rc<Vec<Vec<usize>>>),
    Dep1VecSet(usize, Rc<Vec<HashSet<usize>>>),
    Dep1HashMapVec(usize, Rc<HashMap<usize, Vec<usize>>>),
    Dep1HashMapSet(usize, Rc<HashMap<usize, HashSet<usize>>>),
    Dep1BTreeMapVec(usize, Rc<BTreeMap<usize, Vec<usize>>>),
    Dep1BTreeMapSet(usize, Rc<BTreeMap<usize, HashSet<usize>>>),

    // dependent-dim1 - special
    NextPrevOf(usize, NextPrevBound),
    BoundedBy(usize, Rc<SetElements>, BoundedSet),
    OtherThan(usize, Rc<SetElements>),
}

impl SetElements {
    // todo: this fn will be pub(crate)
    pub fn elements_for(&self, current: &CurrentSetElements) -> Elements<'_> {
        use SetElements::*;
        match self {
            // independent
            Range(x) => Box::new(x.clone()),
            Vec(x) => Box::new(x.iter().cloned()),
            Set(x) => Box::new(x.iter().cloned()),
            DataAndFun(x) => x.elements(),

            // dependent-dim1
            Dep1DataAndFun(i, x) => x.elements(current.at(*i)),
            Dep1VecVec(i, x) => dep1_vec_vec(x, current.at(*i)),
            Dep1VecSet(i, x) => dep1_vec_set(x, current.at(*i)),
            Dep1HashMapVec(i, x) => dep1_hashmap_vec(x, current.at(*i)),
            Dep1HashMapSet(i, x) => dep1_hashmap_set(x, current.at(*i)),
            Dep1BTreeMapVec(i, x) => dep1_btreemap_vec(x, current.at(*i)),
            Dep1BTreeMapSet(i, x) => dep1_btreemap_set(x, current.at(*i)),

            // dependent-dim1 - special
            NextPrevOf(i, x) => x.elements(current.at(*i)),
            BoundedBy(i, set, x) => x.elements(set, current.at(*i), current),
            OtherThan(i, set) => elements_other_than(set, current.at(*i), current),
        }
    }
}
