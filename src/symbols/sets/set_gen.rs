use alloc::boxed::Box;

pub type Elements<S> = Box<dyn Iterator<Item = <S as SetGen>::Element>>;

pub trait SetGen {
    type Element;

    fn elements(&self, set_values: usize) -> Elements<Self>;
}
