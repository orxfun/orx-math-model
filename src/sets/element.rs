pub trait Element {
    type Idx<'a>: PartialEq + Eq
    where
        Self: 'a;

    fn idx<'a>(&'a self) -> Self::Idx<'a>;
}
