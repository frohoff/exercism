use std::cmp::Ordering::{Less, Equal as Equals, Greater};
use Comparison::*;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    fn contains<T: PartialEq>(larger: &[T], smaller: &[T]) -> bool {
        smaller.is_empty() || larger.windows(smaller.len()).any(|w| w == smaller )
    }

    match a.len().cmp(&b.len()) {
        Greater if contains(a, b) => Superlist,
        Less if contains(b, a) => Sublist,
        Equals if a == b => Equal,
        _ => Unequal
    }
}