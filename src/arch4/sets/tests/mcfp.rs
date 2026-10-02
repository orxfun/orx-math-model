use crate::sets::Element;
use std::string::String;

struct Node {
    id: usize,
}

impl Element for Node {
    type Idx<'a> = usize;

    fn idx(&self) -> Self::Idx<'_> {
        self.id
    }
}

struct Edge {
    key: String,
    cost: f64,
    cap: u32,
}

impl Element for Edge {
    type Idx<'a> = &'a str;

    fn idx<'a>(&'a self) -> Self::Idx<'a> {
        self.key.as_str()
    }
}
