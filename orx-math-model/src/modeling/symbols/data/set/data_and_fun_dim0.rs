use super::set_elements::Elements;
use std::rc::Rc;

pub trait DataAndFunSet0 {
    fn elements(&self) -> Elements<'_>;
}

pub struct DataAndFunDim0<Data> {
    pub(super) data: Rc<Data>,
    pub(super) fun: Rc<dyn Fn(&Data) -> Elements<'_>>,
}
impl<Data> DataAndFunSet0 for DataAndFunDim0<Data> {
    fn elements(&self) -> Elements<'_> {
        (self.fun)(&self.data)
    }
}
