use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::symbol_ref::SymbolRef;
use crate::symbols::Set;
use crate::*;
use alloc::vec;
use alloc::vec::Vec;
use std::collections::{BTreeSet, HashSet};

fn depth() -> Depth {
    Depth::zero()
}

fn set_depths<'m>() -> SetDepths<'m> {
    SetDepths::new(core::iter::empty())
}

fn index_values() -> IndexValues {
    IndexValues::new(0)
}

fn elements<'m>() -> Elements<'m> {
    Elements::default()
}

fn get_elements<'m>(set: Set<'m>) -> Vec<usize> {
    let set_gen = SymbolRef::from(set).data.data.set_gen();
    let mut elements = elements();
    set_gen.set_elements(depth(), set_depths(), &index_values(), &mut elements);
    elements.elements(depth()).to_vec()
}

#[test]
fn set_vec() {
    let m = Model::new();
    let i = m.set(vec![0, 2, 4]);
    assert_eq!(&get_elements(i), &[0, 2, 4]);
}

#[test]
fn set_slice() {
    let data = vec![0, 2, 4];
    let slice = data.as_slice();
    let m = Model::new();
    let i = m.set(slice);
    assert_eq!(&get_elements(i), &[0, 2, 4]);
}

#[test]
fn set_range() {
    let m = Model::new();
    let i = m.set(4..9);
    assert_eq!(&get_elements(i), &[4, 5, 6, 7, 8]);
}

#[test]
fn set_hash_set() {
    let m = Model::new();
    let i = m.set(HashSet::from([0, 2, 4]));
    assert_eq!(&get_elements(i), &[0, 2, 4]);
}

#[test]
fn set_btree_set() {
    let m = Model::new();
    let i = m.set(BTreeSet::from([0, 2, 4]));
    assert_eq!(&get_elements(i), &[0, 2, 4]);
}
