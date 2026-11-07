use crate::{sets::elements::Elements, Model};
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn cat_int() {
    let values = vec![1, 2, 7, 9];

    let m = Model::new();
    let set = m.set_cat_int();
    let mut elements = Elements::from(vec![]);
    unsafe { set.el.get() }.as_cat_int().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[]);

    unsafe { set.el.get_mut() }
        .as_cat_int_mut()
        .bind(|| vec![1, 3]);
    unsafe { set.el.get() }.as_cat_int().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[1, 3]);

    unsafe { set.el.get_mut() }
        .as_cat_int_mut()
        .bind(|| values.iter().copied());
    unsafe { set.el.get() }.as_cat_int().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[1, 2, 7, 9]);

    unsafe { set.el.get_mut() }
        .as_cat_int_mut()
        .bind(|| 0..values.len());
    unsafe { set.el.get() }.as_cat_int().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[0, 1, 2, 3]);
}

#[test]
fn cat_str() {
    let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();

    let m = Model::new();
    let set = m.set_cat_str();
    let mut elements = Elements::<&str>::from(vec![]);
    unsafe { set.el.get() }.as_cat_str().fill(&mut elements);
    assert_eq!(&elements, &Elements::from(Vec::new()));

    unsafe { set.el.get_mut() }
        .as_cat_str_mut()
        .bind(|| vec!["1", "3"]);
    unsafe { set.el.get() }.as_cat_str().fill(&mut elements);
    assert_eq!(elements.as_slice(), &["1", "3"]);

    unsafe { set.el.get_mut() }
        .as_cat_str_mut()
        .bind(|| values.iter().map(|x| x.as_str()));
    unsafe { set.el.get() }.as_cat_str().fill(&mut elements);
    assert_eq!(elements.as_slice(), &["1", "2", "7", "9"]);
}

#[test]
fn ord() {
    let values = vec![1, 2, 7, 9];

    let m = Model::new();
    let set = m.set_ord();
    let mut elements = Elements::from(vec![]);
    unsafe { set.el.get() }.as_ord().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[]);

    unsafe { set.el.get_mut() }.as_ord_mut().bind(|| vec![1, 3]);
    unsafe { set.el.get() }.as_ord().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[1, 3]);

    unsafe { set.el.get_mut() }
        .as_ord_mut()
        .bind(|| values.iter().copied());
    unsafe { set.el.get() }.as_ord().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[1, 2, 7, 9]);

    unsafe { set.el.get_mut() }
        .as_ord_mut()
        .bind(|| 0..values.len());
    unsafe { set.el.get() }.as_ord().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[0, 1, 2, 3]);
}
