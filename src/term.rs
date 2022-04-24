use std::cmp::*;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

#[derive(Clone, Debug, Copy)]
pub enum Term<V, O> {
    Var(V),
    Op(O),
}
pub use Term::*;

pub trait Operator: Sized {
    type Var: Ord;
    type ArgIter<'a>: Iterator<Item = &'a Term<Self::Var, Self>>
    where
        Self: 'a;

    /// An iterator over the arguments to an operator.
    fn arg_iter(&self) -> Self::ArgIter<'_>;

    /// A unique number for each distinct operator, used for comparisons.
    fn op_index(&self) -> u64;

    /// Comparison just on the operator, ignoring its arguments.
    fn op_cmp(&self, other: &Self) -> Ordering {
        self.op_index().cmp(&other.op_index())
    }
}

impl<V: Ord, O: Operator<Var = V>> Term<V, O> {
    /// Do an inorder traversal and collect a list of numbers representing the variables. Each
    /// unique variable is mapped to a different number, starting from 1 and increasing in the
    /// order of each variable first appears in the traversal.
    pub fn var_seq(&self) -> Vec<u64> {
        let mut n: u64 = 0;
        let mut varmap: BTreeMap<&V, u64> = BTreeMap::new();
        let mut terms: VecDeque<&Self> = VecDeque::new();
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
                    for a in op.arg_iter() {
                        terms.push_back(a);
                    }
                }
            }
        }
        vars
    }

    /// Determine the comparison of two terms, using the operator structure, ignoring variables.
    pub fn varop_cmp(&self, other: &Term<V, O>) -> Ordering {
        match (self, other) {
            (Var(_), Var(_)) => Ordering::Equal,
            (Var(_), Op(_)) => Ordering::Less,
            (Op(_), Var(_)) => Ordering::Greater,
            (Op(f), Op(g)) => {
                let op_ordering = f.op_cmp(g);
                if op_ordering == Ordering::Equal {
                    f.arg_iter().cmp_by(g.arg_iter(), |ft, gt| ft.varop_cmp(gt))
                } else {
                    op_ordering
                }
            }
        }
    }
}

impl<V: Ord, O: Operator<Var = V>> PartialEq for Term<V, O> {
    /// Two terms are equal if they have the same operator structure and the variable sequence.
    fn eq(&self, other: &Term<V, O>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<V: Ord, O: Operator<Var = V>> Eq for Term<V, O> {}

impl<V: Ord, O: Operator<Var = V>> PartialOrd for Term<V, O> {
    fn partial_cmp(&self, other: &Term<V, O>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V: Ord, O: Operator<Var = V>> Ord for Term<V, O> {
    fn cmp(&self, other: &Term<V, O>) -> Ordering {
        let varop_ord = self.varop_cmp(other);
        if varop_ord == Ordering::Equal {
            self.var_seq().cmp(&other.var_seq())
        } else {
            varop_ord
        }
    }
}

impl<V: fmt::Display, O: fmt::Display> fmt::Display for Term<V, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var(v) => v.fmt(f),
            Op(o) => o.fmt(f),
        }
    }
}

// TODO: implement common-subterm search
