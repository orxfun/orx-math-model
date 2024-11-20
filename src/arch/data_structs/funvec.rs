use std::rc::Rc;

pub type GetElemFromData<const D: usize, Data, Elem> = dyn Fn(&Data, [usize; D]) -> Elem;

pub trait FunVec<const D: usize, Elem> {
    fn value(&self, indices: [usize; D]) -> Elem;
}

pub struct DataAndFunVec<const D: usize, Data, Elem> {
    data: Rc<Data>,
    fun: Rc<GetElemFromData<D, Data, Elem>>,
}

impl<const D: usize, Data, Elem> FunVec<D, Elem> for DataAndFunVec<D, Data, Elem> {
    fn value(&self, indices: [usize; D]) -> Elem {
        (self.fun)(&self.data, indices)
    }
}

impl<const D: usize, Data, Fun, Elem> From<(Rc<Data>, Fun)> for DataAndFunVec<D, Data, Elem>
where
    Fun: for<'a> Fn(&'a Data, [usize; D]) -> Elem + 'static,
{
    fn from(value: (Rc<Data>, Fun)) -> Self {
        Self {
            data: value.0,
            fun: Rc::new(value.1),
        }
    }
}
