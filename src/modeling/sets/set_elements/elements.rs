use crate::modeling::current_set_elements::CurrentSetElements;

pub trait Elements {
    fn elements(&self, current: &CurrentSetElements) -> impl Iterator<Item = usize>;
}

impl<'a, E: Elements> Elements for &'a E {
    fn elements(&self, current: &CurrentSetElements) -> impl Iterator<Item = usize> {
        <E as Elements>::elements(self, current)
    }
}

impl<'a, E: Elements> Elements for &'a mut E {
    fn elements(&self, current: &CurrentSetElements) -> impl Iterator<Item = usize> {
        <E as Elements>::elements(self, current)
    }
}
