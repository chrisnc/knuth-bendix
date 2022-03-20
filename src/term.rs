use std::cmp::*;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

pub trait Operator<'a>: Sized {
    type Var: Ord;
    type Op: PartialEq;

    fn args(&'a self) -> &'a [&'a Term<Self::Var, Self>];

    fn opeq(&self, other: &Self) -> bool;
}

#[derive(Clone, Debug)]
pub enum Term<V, O> {
    Var(V),
    Op(O),
}
pub use Term::*;

/*
impl<'a, T: Term<'a>> PartialEq for T {
    fn eq(&'a self, other: &T) -> bool {
        false
    }
}
*/

impl<'a, V: Ord, O: Operator<'a, Var = V>> Term<V, O> {
    // Assign incrementing numbers to each variable encountered, returning the same number if the
    // same variable appears later. Traversal is inorder.
    pub fn varseq(&'a self) -> Vec<u32> {
        let mut n: u32 = 0;
        let mut varmap: BTreeMap<&V, u32> = BTreeMap::new();
        let mut terms: VecDeque<&'a Self> = VecDeque::new();
        let mut vars: Vec<u32> = vec![];
        terms.push_back(self);
        while let Some(t) = terms.pop_front() {
            match t {
                Var(var) => {
                    vars.push(*varmap.entry(var).or_insert_with(|| {
                        // If the variable hasn't been seen yet, assign a new number to it.
                        n += 1;
                        n
                    }));
                }
                Op(op) => {
                    for a in op.args() {
                        terms.push_back(*a);
                    }
                }
            }
        }
        vars
    }
}

impl<'a, V: fmt::Display, O: Operator<'a> + fmt::Display> fmt::Display for Term<V, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var(v) => write!(f, "{}", v),
            Op(o) => write!(f, "{}", o),
        }
    }
}

/*
impl<V: Ord, O: PartialEq + Op<V>> PartialEq for Term<V, O> {
    fn eq(&self, other: &Term<V, O>) -> bool {
        // First determine if the terms have the same operators.
        (match (self, other) {
            (Variable(_), Variable(_)) => true,
            (Operator(f), Operator(g)) => f.opeq(g) && f.args().zip(g.args()).all(|(ft, gt)| ft.eq(gt)),
            _ => false,
        }) && self.varseq() == other.varseq()
    }
}

// TODO: are these the right trait bounds?
impl<V: Ord, O: Op<V> + Ord> PartialOrd for Term<V, O> {
    fn partial_cmp(&self, _other: &Term<V, O>) -> Option<Ordering> {
        // TODO
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::prod::*;

    #[test]
    fn display() {
        let m = Prod::mul(Prod::var("a"), Prod::var("b"));
        println!("{}", m);
    }

    #[test]
    fn eq() {
        let tab = Prod::mul(Prod::var("a"), Prod::var("b"));
        let tcd = Prod::mul(Prod::var("c"), Prod::var("d"));
        let taa = Prod::mul(Prod::var("a"), Prod::var("a"));
        let tbb = Prod::mul(Prod::var("b"), Prod::var("b"));
        assert_eq!(tab, tcd);
        assert_eq!(taa, tbb);
        assert!(!(taa == tab));
    }
}
*/
