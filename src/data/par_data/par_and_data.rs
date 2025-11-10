use crate::Par;

pub trait ParAndData<'m, const N: usize> {
    fn par(&self) -> Par<'m, N>;

    // fn elements(
    //     &'m self,
    //     set_depths: &SetDepths<'m>,
    //     index_values: &IndexValues,
    // ) -> Box<dyn Iterator<Item = usize> + '_> {
    //     self.set_gen()
    //         .elements(self.set(), set_depths, index_values)
    // }
}
