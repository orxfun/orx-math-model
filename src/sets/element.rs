pub trait Element {
    type Idx: PartialEq + Eq;

    fn idx(&self) -> Self::Idx;
}
