use std::cmp::Ordering::{Less, Equal as Equals, Greater};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    fn contains<T: PartialEq>(larger: &[T], smaller: &[T]) -> bool {
        (0..(larger.len() - smaller.len() + 1)).any(|i| larger[i..(i+smaller.len())] == *smaller)
    }

    match a.len().cmp(&b.len()) {
        Greater if contains(a, b) => Superlist,
        Less if contains(b, a) => Sublist,
        Equals if a == b => Equal,
        _ => Unequal
    }
}