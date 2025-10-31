// use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
// use crate::symbols::{sets::set_gen::SetGen, Set};

// pub struct DependentSet<'m, const N: usize, I, E>
// where
//     I: IntoIterator<Item = usize>,
//     E: Fn(&[usize], [usize; N]) -> I,
// {
//     parent: Set<'m>,
//     dependencies: [Set<'m>; N],
//     elements: E,
// }

// impl<'a, const N: usize, I, E> SetGen for DependentSet<'a, N, I, E>
// where
//     I: IntoIterator<Item = usize>,
//     E: Fn(&[usize], [usize; N]) -> I,
// {
//     fn set_elements<'m>(
//         &'m self,
//         depth: Depth,
//         set_depths: SetDepths<'_>,
//         index_values: &IndexValues,
//         elements: &'m mut Elements<'m>,
//     ) {
//         let parent_depth = set_depths.depth_of(self.parent);
//         let parent_elements = elements.parent_elements(depth, parent_depth);

//         let dependency_depths = self.dependencies.map(|p| set_depths.depth_of(p));
//         let dependency_indices = index_values.values(dependency_depths);

//         let dependent_elements = (self.elements)(parent_elements, dependency_indices);
//         elements.set_stored_elements(depth, dependent_elements);
//     }
// }
