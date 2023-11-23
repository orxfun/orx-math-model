pub trait DefiningStage {}

pub struct AddingConstraints;
impl DefiningStage for AddingConstraints {}

pub struct AddingWeightedObjectives;
impl DefiningStage for AddingWeightedObjectives {}

pub struct ModelDefined;
impl DefiningStage for ModelDefined {}
