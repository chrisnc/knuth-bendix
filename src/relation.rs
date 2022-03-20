use std::fmt;

use crate::Term;

#[derive(Clone)]
pub struct Relation<V, O> {
    left: Term<V, O>,
    right: Term<V, O>,
}

impl fmt::Display for Relation<String, String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}
