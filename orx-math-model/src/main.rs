use mcnf::{Mcnf, SoaMcnfGraph};
use modeling::{stages::Building, symbols::constraint::Constraint};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub mod data_structs;
mod mcnf;
pub mod modeling;
pub mod numerics;
pub mod str_expr;

fn print<X: Debug + Display>(x: &X) {
    println!();
    println!("{}", x);
    println!();
    println!("{:?}", x);
    println!();
}
// fn consume_data(_: Model<Built>) {}

#[allow(clippy::precedence)]
fn main() {
    let graph = Rc::new(SoaMcnfGraph::default());
    let mcnf = Mcnf::new(graph).unwrap();

    println!("{}", mcnf.x);

    println!("{:?}", mcnf.model);
}

fn get_constraint<C: Into<Constraint<Building>> + Debug + Display>(c: C) {
    let c: Constraint<Building> = c.into();
    print(&c)
}
