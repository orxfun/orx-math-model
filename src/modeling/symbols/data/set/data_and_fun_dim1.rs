use super::set_elements::Elements;
use std::rc::Rc;

type GetElementsDim1<Data> = dyn Fn(&Data, usize) -> Elements<'_>;

pub trait DataAndFunSet1 {
    fn elements(&self, i: usize) -> Elements<'_>;
}

pub struct DataAndFunDim1<Data> {
    pub(super) data: Rc<Data>,
    pub(super) fun: Rc<GetElementsDim1<Data>>,
}
impl<Data> DataAndFunSet1 for DataAndFunDim1<Data> {
    fn elements(&self, i: usize) -> Elements<'_> {
        (self.fun)(&self.data, i)
    }
}
