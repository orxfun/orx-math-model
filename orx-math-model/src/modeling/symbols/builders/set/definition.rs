use super::other_than::SetDefinitionOtherThan;
use super::{bounded_by::SetDefinitionBoundedBy, depends_on::SetDefinitionDependsOn};
use crate::modeling::stages::Building;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::modeling::symbols::data::set::from::indep::{
    IntoSetDataAndDataIndependent, IntoSetDataIndependent,
};
use crate::modeling::symbols::data::set::next_prev_of::NextPrevBound;
use crate::modeling::symbols::data::set::set_elements::SetElements;
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::{
        modeler::{Modeler, SymbolFromBuilder},
        reference::{HasRef, SymRef},
        symbols::{
            data::{
                scalar_data::{ScalarData, ScalarVariant},
                set::bounded_by::{BoundEnd, BoundFrom, BoundedSet},
                set_data::SetData,
            },
            scalar::Scalar,
            set::Set,
            symbol::Symbol,
        },
    },
    numerics::num::RealNum,
};
use std::ops::Range;
use std::rc::Rc;

pub struct SetDefinition {
    modeler: Modeler,
    symbol: Symbol,
}

impl SetDefinition {
    pub(crate) fn new(modeler: Modeler, symbol: Symbol) -> Self {
        Self { modeler, symbol }
    }

    pub(super) fn push_get_set(self, elements: SetElements) -> Set<Building> {
        fn push_get_set_value_scalar(set_ref: &SymRef<Building>, key: &str) -> SymRef<Building> {
            let set_idx = set_ref.sym_idx;
            let str = key.to_string();
            let fun = Rc::new(move |indices: &CurrentSetElements| indices.at(set_idx) as RealNum);

            let symref = set_ref.core_ref.new_symref_scalar();
            let variant = ScalarVariant::Func { str, fun };
            let symbol = Scalar::new(symref);
            let storage = ScalarData { symbol, variant };

            *set_ref.core().scalars.push(storage).sym_ref()
        }

        let symref = self.modeler.new_index(SymbolFromBuilder::Set);
        let (key, definition) = self.symbol.into_key_definition();

        let scalar_ref = push_get_set_value_scalar(&symref, &key);
        let symbol = Set::new(symref);

        let storage = SetData {
            symbol,
            elements,
            scalar_ref,
            key,
            definition,
        };

        self.modeler.data.sets.push(storage)
    }

    // indep
    pub fn from_range(self, range: Range<usize>) -> Set<Building> {
        let elements = SetElements::Range(range);
        self.push_get_set(elements)
    }
    pub fn from_data<Data>(self, data: Data) -> Set<Building>
    where
        Data: IntoSetDataIndependent,
    {
        let elements = data.into_set_elements();
        self.push_get_set(elements)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Set<Building>
    where
        (Rc<Data>, Fun): IntoSetDataAndDataIndependent<Data>,
    {
        let elements = (data, fun).into_set_elements();
        self.push_get_set(elements)
    }

    // next-prev
    pub fn next_of(
        self,
        dependent_set: Set<Building>,
        maximum_allowed_value: Option<usize>,
    ) -> Set<Building> {
        Self::next_prev_of(self, dependent_set, maximum_allowed_value, true)
    }
    pub fn prev_of(
        self,
        dependent_set: Set<Building>,
        minimum_allowed_value: Option<usize>,
    ) -> Set<Building> {
        Self::next_prev_of(self, dependent_set, minimum_allowed_value, false)
    }
    fn next_prev_of(
        self,
        dependent_set: Set<Building>,
        bounded_by: Option<usize>,
        is_next: bool,
    ) -> Set<Building> {
        use NextPrevBound::*;
        let depending_set_idx = dependent_set.sym_idx();
        let bound = if is_next {
            NextUntilInclusive(bounded_by.unwrap_or(usize::MAX))
        } else {
            PrevAfterInclusive(bounded_by.unwrap_or(0))
        };
        let elements = SetElements::NextPrevOf(depending_set_idx, bound);
        self.push_get_set(elements)
    }

    // depends on
    pub fn depends_on(self, dependent_set: Set<Building>) -> SetDefinitionDependsOn {
        SetDefinitionDependsOn {
            definition: self,
            depending_set_idx: dependent_set.sym_idx(),
        }
    }

    // other than
    pub fn other_than(self, other_than_set: Set<Building>) -> SetDefinitionOtherThan {
        SetDefinitionOtherThan {
            definition: self,
            other_than_set_idx: other_than_set.sym_idx(),
        }
    }

    // bounded
    fn into_bounded_by(
        self,
        dependent_set: Set<Building>,
        bound_from: BoundFrom,
        bound_type: BoundEnd,
    ) -> SetDefinitionBoundedBy {
        SetDefinitionBoundedBy {
            definition: self,
            bounding_set_idx: dependent_set.sym_idx(),
            bounded_set: BoundedSet {
                bound_from,
                bound_type,
            },
        }
    }
    pub fn less_than(self, dependent_set: Set<Building>) -> SetDefinitionBoundedBy {
        self.into_bounded_by(dependent_set, BoundFrom::Above, BoundEnd::Exclusive)
    }
    pub fn less_than_or_eq(self, dependent_set: Set<Building>) -> SetDefinitionBoundedBy {
        self.into_bounded_by(dependent_set, BoundFrom::Above, BoundEnd::Inclusive)
    }
    pub fn greater_than(self, dependent_set: Set<Building>) -> SetDefinitionBoundedBy {
        self.into_bounded_by(dependent_set, BoundFrom::Below, BoundEnd::Exclusive)
    }
    pub fn greater_than_or_eq(self, dependent_set: Set<Building>) -> SetDefinitionBoundedBy {
        self.into_bounded_by(dependent_set, BoundFrom::Below, BoundEnd::Inclusive)
    }
}
