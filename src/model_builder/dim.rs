pub trait Dim {
    type Next: Dim;

    type Arr<T>;

    fn arr_push<T>(arr: Self::Arr<T>, value: T) -> <Self::Next as Dim>::Arr<T>
    where
        T: Clone;
}

pub enum Never {}
impl Dim for Never {
    type Next = Self;

    type Arr<T> = ();

    fn arr_push<T>(_: Self::Arr<T>, _: T) -> <Self::Next as Dim>::Arr<T>
    where
        T: Clone,
    {
        unreachable!()
    }
}

// dim 0

pub struct D0;
impl Dim for D0 {
    type Next = D1;

    type Arr<T> = [T; 0];

    fn arr_push<T>(_: Self::Arr<T>, value: T) -> <Self::Next as Dim>::Arr<T>
    where
        T: Clone,
    {
        [value]
    }
}

// dim 1

pub struct D1;
impl Dim for D1 {
    type Next = D2;

    type Arr<T> = [T; 1];

    fn arr_push<T>(arr: Self::Arr<T>, value: T) -> <Self::Next as Dim>::Arr<T>
    where
        T: Clone,
    {
        [arr[0].clone(), value]
    }
}

// dim 2

pub struct D2;
impl Dim for D2 {
    type Next = D3;

    type Arr<T> = [T; 2];

    fn arr_push<T>(arr: Self::Arr<T>, value: T) -> <Self::Next as Dim>::Arr<T>
    where
        T: Clone,
    {
        [arr[0].clone(), arr[1].clone(), value]
    }
}

// dim 3

pub struct D3;
impl Dim for D3 {
    type Next = Never;

    type Arr<T> = [T; 3];

    fn arr_push<T>(arr: Self::Arr<T>, value: T) -> <Self::Next as Dim>::Arr<T>
    where
        T: Clone,
    {
        unreachable!()
    }
}
