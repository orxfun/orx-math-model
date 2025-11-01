pub trait Stage {}

pub struct Modeling;
impl Stage for Modeling {}

pub struct ModelWithData;
impl Stage for ModelWithData {}

pub struct Solved;
impl Stage for Solved {}
