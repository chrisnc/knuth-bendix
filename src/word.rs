use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display};

pub trait Variable: Eq + Ord + Clone {}

impl Variable for String {}

pub trait Operator: Eq + Clone {
    fn min_weight() -> u64;
    fn arity(&self) -> usize;
    fn weight(&self) -> u64;
    fn op_index(&self) -> u64;
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol<V, O> {
    Var(V),
    Op(O),
}
pub use Symbol::*;

impl<V, O: Operator> Symbol<V, O> {
    fn arity(&self) -> usize {
        if let Op(f) = self {
            f.arity()
        } else {
            0
        }
    }
}

impl<V, O> From<V> for Symbol<V, O> {
    fn from(v: V) -> Symbol<V, O> {
        Var(v)
    }
}

impl<V: Variable, O: Operator> Symbol<V, O> {
    fn weight(&self) -> u64 {
        match self {
            Var(_) => O::min_weight(),
            Op(f) => f.weight(),
        }
    }
}

impl<V: Display, O: Display> Display for Symbol<V, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var(v) => v.fmt(f),
            Op(o) => o.fmt(f),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Word<V, O> {
    pub syms: Vec<Symbol<V, O>>,
}

impl<V: Variable, O: Operator> Word<V, O> {
    pub fn from_sym(s: Symbol<V, O>) -> Word<V, O> {
        Word { syms: vec![s] }
    }

    pub fn var<VF: Into<V>>(v: VF) -> Word<V, O> {
        Word::from_sym(Var(v.into()))
    }

    pub fn op<OF: Into<O>>(f: OF, args: &[Word<V, O>]) -> Word<V, O> {
        let mut out = Word::from_sym(Op(f.into()));
        for a in args {
            out.syms.extend(a.syms.clone());
        }
        out
    }

    fn weight(&self) -> u64 {
        self.syms.iter().map(Symbol::weight).sum()
    }

    fn n(&self, var: &V) -> usize {
        let v = Var(var.clone());
        self.syms.iter().filter(|s| **s == v).count()
    }

    fn vars(&self) -> BTreeSet<V> {
        self.syms
            .iter()
            .filter_map(|s| match s {
                Var(v) => Some(v),
                Op(_) => None,
            })
            .cloned()
            .collect()
    }

    pub fn is_well_formed(&self) -> bool {
        let mut nsyms: isize = 1;
        for s in self.syms.iter() {
            nsyms += (s.arity() as isize) - 1;
        }
        nsyms == 0
    }

    pub fn subwords(&self) -> Subwords<'_, V, O> {
        Subwords {
            syms: &self.syms,
            i: 1,
            nargs: self.syms.first().map_or(0, Symbol::arity),
        }
    }

    pub fn subst(&mut self, vars: &BTreeMap<V, Word<V, O>>) {
        for (v, w) in vars.iter() {
            while let Some(i) =
                self.syms
                    .iter()
                    .position(|s| if let Var(x) = s { x == v } else { false })
            {
                self.syms.splice(i..i + 1, w.syms.clone());
            }
        }
    }
}

pub struct Subwords<'a, V, O> {
    syms: &'a Vec<Symbol<V, O>>,
    i: usize,
    nargs: usize,
}

impl<'a, V: Variable, O: Operator> Iterator for Subwords<'a, V, O> {
    type Item = Word<V, O>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.nargs > 0 {
            self.nargs -= 1;
            let mut nsyms: usize = 1;
            let swstart = self.i;
            while nsyms > 0 {
                nsyms -= 1;
                nsyms += self.syms.get(self.i).map_or(0, Symbol::arity);
                self.i += 1;
            }
            self.syms.get(swstart..self.i).map(|subsyms| Word {
                syms: Vec::from(subsyms),
            })
        } else {
            None
        }
    }
}

impl<V: Variable, O: Operator> PartialEq for Word<V, O> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<V: Variable, O: Operator> PartialOrd for Word<V, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Case 1
        // w(alpha) > w(beta) and n(vi, alpha) >= n(vi, beta) for all vi
        // Each variable must occur at least as often in alpha as in beta.
        let sw = self.weight();
        let ow = other.weight();
        let vars: BTreeSet<V> = self.vars().union(&other.vars()).cloned().collect();
        if sw > ow {
            for v in vars.iter() {
                if self.n(v) < other.n(v) {
                    return None;
                }
            }
            Some(Ordering::Greater)
        // Case 2 from Knuth-Bendix
        // w(alpha) == w(beta) and n(vi, alpha) == n(vi, beta) for all vi
        // Each variable must occur exactly as often in alpha as in beta, otherwise equal
        // weight words can't be compared.
        } else if sw == ow {
            for v in vars.iter() {
                if self.n(v) != other.n(v) {
                    return None;
                }
            }
            match (self.syms.first(), other.syms.first()) {
                // This covers the case where self is f^N x and other is x. Each word has exactly
                // the same variables here, so if one side is just a variable, then the other side
                // also only has one of that same variable. They also have the same weight, which
                // means that there are no operators which have positive weight, otherwise the only
                // side with any operators would have larger weight. Operators with arity 2 or more
                // may have zero weight, but this would ultimately require additional variables or
                // nullary operators on one side, which would contribute to a larger weight, and so
                // can't happen here. Therefore the only operator in play here is a unary operator
                // of zero weight, and the ordering defines this to mean that the one with an
                // operator is greater than the one without.
                (Some(Op(_)), Some(Var(_))) => Some(Ordering::Greater),
                (Some(Var(_)), Some(Op(_))) => Some(Ordering::Less),

                // We already know these are the same variable from comparing n(v) for all
                // variables appearing in either word. If they are different variables then
                // None is returned in that loop.
                (Some(Var(_)), Some(Var(_))) => Some(Ordering::Equal),

                (Some(Op(f)), Some(Op(g))) => {
                    if f.op_index() > g.op_index() {
                        Some(Ordering::Greater)
                    } else if f.op_index() == g.op_index() {
                        self.subwords().partial_cmp(other.subwords())
                    } else {
                        Some(Ordering::Less)
                    }
                }
                _ => None,
            }
        // Case 1 but in the opposite direction.
        } else {
            for v in vars.iter() {
                if self.n(v) > other.n(v) {
                    return None;
                }
            }
            Some(Ordering::Less)
        }
    }
}

// TODO: implement common-subterm search
