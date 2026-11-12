use crate::{new_bag, sets::SetTyp, Bag, Elements, Res, Slot};
use alloc::{boxed::Box, string::String};

pub enum SetFun<'d> {
    CatSetInt(Box<dyn Fn(&mut Elements<usize>) + 'd>),
    CatSetStr(Box<dyn Fn(&mut Elements<&'d String>) + 'd>),
    OrdSet(Box<dyn Fn(&mut Elements<i64>) + 'd>),
}

#[derive(Clone)]
pub enum SetSlot<'d> {
    CatSetInt(Slot<'d, Box<dyn Fn(&mut Elements<usize>) + 'd>>),
    CatSetStr(Slot<'d, Box<dyn Fn(&mut Elements<&'d String>) + 'd>>),
    OrdSet(Slot<'d, Box<dyn Fn(&mut Elements<i64>) + 'd>>),
}

#[derive(derive_more::From)]
pub enum SetElements<'a, 'd> {
    CatSetInt(&'a mut Elements<usize>),
    CatSetStr(&'a mut Elements<&'d String>),
    OrdSet(&'a mut Elements<i64>),
}

impl<'d> SetSlot<'d> {
    pub fn bind(&self, fun: SetFun<'d>) -> Res {
        match (self, fun) {
            (Self::CatSetInt(s), SetFun::CatSetInt(f)) => s.bind(f),
            (Self::CatSetStr(s), SetFun::CatSetStr(f)) => s.bind(f),
            (Self::OrdSet(s), SetFun::OrdSet(f)) => s.bind(f),
            _ => unreachable!("TODO: explain why"),
        }
    }

    pub fn fill<'a>(&self, elements: impl Into<SetElements<'a, 'd>>) -> Res
    where
        'd: 'a,
    {
        let elements = elements.into();
        match (self, elements) {
            (Self::CatSetInt(s), SetElements::CatSetInt(e)) => {
                s.assert_is_bound()?;
                (s.get())(e);
                Ok(())
            }
            (Self::CatSetStr(s), SetElements::CatSetStr(e)) => {
                s.assert_is_bound()?;
                (s.get())(e);
                Ok(())
            }
            (Self::OrdSet(s), SetElements::OrdSet(e)) => {
                s.assert_is_bound()?;
                (s.get())(e);
                Ok(())
            }
            _ => unreachable!("TODO: explain why"),
        }
    }
}

// collection

pub struct SetSlots<'d> {
    cat_set_int: Bag<Box<dyn Fn(&mut Elements<usize>) + 'd>>,
    cat_set_str: Bag<Box<dyn Fn(&mut Elements<&'d String>) + 'd>>,
    ord_set: Bag<Box<dyn Fn(&mut Elements<i64>) + 'd>>,
}

impl Default for SetSlots<'_> {
    fn default() -> Self {
        Self {
            cat_set_int: new_bag(),
            cat_set_str: new_bag(),
            ord_set: new_bag(),
        }
    }
}

impl<'d> SetSlots<'d> {
    pub fn push(&self, t: SetTyp) -> SetSlot<'d> {
        match t {
            SetTyp::CatSetInt => {
                let el = self.cat_set_int.push(Box::new(|_| {}));
                SetSlot::CatSetInt(el.into())
            }
            SetTyp::CatSetStr => {
                let el = self.cat_set_str.push(Box::new(|_| {}));
                SetSlot::CatSetStr(el.into())
            }
            SetTyp::OrdSet => {
                let el = self.ord_set.push(Box::new(|_| {}));
                SetSlot::OrdSet(el.into())
            }
        }
    }
}
