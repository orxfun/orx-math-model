use std::fmt::Display;

#[allow(non_camel_case_types)]
pub struct minimize;
#[allow(non_camel_case_types)]
pub struct maximize;
pub(crate) trait ObjectiveDirectionMarker {
    fn direction(&self) -> ObjectiveDirection;
}
impl ObjectiveDirectionMarker for minimize {
    fn direction(&self) -> ObjectiveDirection {
        ObjectiveDirection::Minimize
    }
}
impl ObjectiveDirectionMarker for maximize {
    fn direction(&self) -> ObjectiveDirection {
        ObjectiveDirection::Maximize
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ObjectiveDirection {
    Minimize,
    Maximize,
}

impl Display for ObjectiveDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minimize => write!(f, "min"),
            Self::Maximize => write!(f, "max"),
        }
    }
}
