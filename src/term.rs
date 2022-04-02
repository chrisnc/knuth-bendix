use std::cmp::*;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

pub trait Operator<'a>: Sized {
    type Var: Ord;
    type Op: PartialEq;
    type Args: Iterator<Item = &'a Term<Self::Var, Self>>
    where
        Self: 'a;

    fn args(&'a self) -> Self::Args;

    fn opeq(&self, other: &Self) -> bool;
}

#[derive(Clone, Debug)]
pub enum Term<V, O> {
    Var(V),
    Op(O),
}
pub use Term::*;

impl<'a, V: Ord, O: Operator<'a, Var = V>> Term<V, O> {
    // Assign incrementing numbers to each variable encountered, returning the same number if the
    // same variable appears later. Traversal is inorder.
    pub fn varseq(&'a self) -> Vec<u64> {
        let mut n: u64 = 0;
        let mut varmap: BTreeMap<&V, u64> = BTreeMap::new();
        let mut terms: VecDeque<&'a Self> = VecDeque::new();
        let mut vars: Vec<u64> = vec![];
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
                        terms.push_back(a);
                    }
                }
            }
        }
        vars
    }

    // Determine if two terms have the same operator structure, ignoring variables.
    pub fn varopeq<'b: 'a>(&'a self, other: &'b Term<V, O>) -> bool {
        match (self, other) {
            (Var(_), Var(_)) => true,
            (Op(f), Op(g)) => f.opeq(g) && f.args().zip(g.args()).all(|(ft, gt)| ft.varopeq(gt)),
            _ => false,
        }
    }

    // TODO: figure out how to implement the eq from PartialEq. For now, the lifetimes don't match.
    pub fn eq<'b: 'a>(&'a self, other: &'b Term<V, O>) -> bool {
        self.varopeq(other) && self.varseq() == other.varseq()
    }
}

/*
impl<'a, 'l: 'a, 'r: 'a, V: Ord, O: Operator<'a, Var = V>> PartialEq for Term<V, O> {
    fn eq(&'l self, other: &'r Term<V, O>) -> bool {
        self.eq_explicit(other)
    }
}
*/

/*
impl<'a, V: Ord, O: Operator<'a, Var = V>> PartialEq for Term<V, O> {
    fn eq(&self, other: &Term<V, O>) -> bool {
        self.eq_explicit(other)
    }
}
*/

impl<'a, V: fmt::Display, O: Operator<'a> + fmt::Display> fmt::Display for Term<V, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var(v) => write!(f, "{}", v),
            Op(o) => write!(f, "{}", o),
        }
    }
}

/*
impl<'a, V: Ord, O: PartialEq + Operator<'a, Var = V>> PartialEq for Term<V, O> {
    fn eq<'b, 'c>(&'b self, other: &'c Term<V, O>) -> bool where 'b: 'a, 'c: 'a {
        // First determine if the terms have the same operators.
        (match (self, other) {
            (Var(_), Var(_)) => true,
            (Op(f), Op(g)) => f.opeq(g) && f.args().zip(g.args()).all(|(ft, gt)| ft.eq(gt)),
            _ => false,
        }) && self.varseq() == other.varseq()
    }
}
*/

/*
// TODO: are these the right trait bounds?
impl<'a, V: Ord, O: Operator<'a, Var = V> + Ord> PartialOrd for Term<V, O> {
    fn partial_cmp(&self, _other: &Term<V, O>) -> Option<Ordering> {
        // TODO
        None
    }
}
*/

#[cfg(test)]
mod tests {
    use crate::prod::*;

    #[test]
    fn display() {
        let a = Prod::var("a");
        let b = Prod::var("b");
        let m = Prod::mul(&a, &b);
        println!("{}", m);
    }

    #[test]
    fn eq() {
        let a = Prod::var("a");
        let b = Prod::var("b");
        let c = Prod::var("c");
        let d = Prod::var("d");
        let tab = Prod::mul(&a, &b);
        let tcd = Prod::mul(&c, &d);
        let taa = Prod::mul(&a, &a);
        let tbb = Prod::mul(&b, &b);
        assert!(tab.eq(&tcd));
        assert!(taa.eq(&tbb));
        assert!(!taa.eq(&tab));
    }
}
