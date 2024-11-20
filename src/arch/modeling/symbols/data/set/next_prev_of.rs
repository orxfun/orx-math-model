use super::set_elements::Elements;

#[derive(Clone)]
pub enum NextPrevBound {
    NextUntilInclusive(usize),
    PrevAfterInclusive(usize),
}

impl NextPrevBound {
    fn get_nextprev(&self, dependent_set_element: usize) -> Option<usize> {
        use NextPrevBound::*;
        match self {
            NextUntilInclusive(x) => {
                if dependent_set_element == *x {
                    None
                } else {
                    Some(x + 1)
                }
            }
            PrevAfterInclusive(x) => {
                if dependent_set_element == *x {
                    None
                } else {
                    Some(x - 1)
                }
            }
        }
    }
    pub(crate) fn elements(&self, dependent_set_element: usize) -> Elements<'_> {
        Box::new(self.get_nextprev(dependent_set_element).into_iter())
    }
}
