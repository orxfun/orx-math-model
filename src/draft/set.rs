pub trait Set: Clone + Copy {
    type Key;
    type Elem;
}

pub trait Set0: Set {}

pub trait Set1: Set {
    type S0: Set0;
}
