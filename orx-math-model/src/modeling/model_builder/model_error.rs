#[derive(Debug, Clone)]
pub enum ModelError {
    EmptyModel,
    AlienConstraint(String),
    AlienObjective(String),
    AlienSymbol,
    AlienVariable(String),
}
