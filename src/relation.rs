/*
use std::fmt;

use crate::{Term};

#[derive(Clone)]
pub struct Relation<V, O: Op<V>> {
    left: Term<V, O>,
    right: Term<V, O>,
}

impl<V: fmt::Display, O: Op<V> + fmt::Display> fmt::Display for Relation<V, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}
*/
