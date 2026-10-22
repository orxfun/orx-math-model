/// Implements `Elements::elements` trait by calling `self.iter().copied()`.
#[macro_export]
macro_rules! impl_elements_from_iter_copied {
    // ($V:ty) => {
    //     impl crate::modeling::sets::set_elements::elements::Elements for $V {
    //         fn elements(
    //             &self,
    //             _: &crate::modeling::current_set_elements::CurrentSetElements,
    //         ) -> impl Iterator<Item = usize> {
    //             self.iter().copied()
    //         }
    //     }
    // };

    ([$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        impl<$($impl_generics)*> crate::modeling::sets::set_elements::elements::Elements for $V where $($where)* {
            fn elements(
                &self,
                _: &crate::modeling::current_set_elements::CurrentSetElements,
            ) -> impl Iterator<Item = usize> {
                self.iter().copied()
            }
        }
    };

    ($const_arg:tt, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        impl<const $const_arg: usize, $($impl_generics)*> crate::modeling::sets::set_elements::elements::Elements for $V where $($where)* {
            fn elements(
                &self,
                _: &crate::modeling::current_set_elements::CurrentSetElements,
            ) -> impl Iterator<Item = usize> {
                self.iter().copied()
            }
        }
    };

    // ($const_arg:tt, $V:ty) => {
    //     #[allow(unused_imports)]
    //     use $crate::*;

    //     impl<const $const_arg: usize> crate::modeling::sets::set_elements::elements::Elements
    //         for $V
    //     {
    //         fn elements(
    //             &self,
    //             _: &crate::modeling::current_set_elements::CurrentSetElements,
    //         ) -> impl Iterator<Item = usize> {
    //             self.iter().copied()
    //         }
    //     }
    // };
}
