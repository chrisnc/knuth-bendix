use std::cmp::*;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Term<V, O> {
    Variable { var: V },
    Operator { op: O, args: Vec<Term<V, O>> },
}
pub use Term::*;

impl<V: fmt::Display, O: fmt::Display> fmt::Display for Term<V, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variable { var: v } => write!(f, "{}", v),
            Operator { op, args } => write!(
                f,
                "{}({})",
                op,
                args.iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl<V, O> Term<V, O> {
    pub fn var<IntoV: Into<V>>(var: IntoV) -> Term<V, O> {
        Term::Variable { var: var.into() }
    }

    pub fn op<IntoO: Into<O>>(op: IntoO, args: Vec<Term<V, O>>) -> Term<V, O> {
        Term::Operator { op: op.into(), args }
    }

    pub fn is_var(&self) -> bool {
        match self {
            Variable { var: _ } => true,
            _ => false,
        }
    }

    pub fn is_op(&self) -> bool {
        match self {
            Operator { op: _, args: _ } => true,
            _ => false,
        }
    }
}

pub trait Arity {
    fn arity(&self) -> usize;
}

impl<V, O: Arity> Term<V, O> {
    pub fn is_well_formed(&self) -> bool {
        match self {
            Variable { var: _ } => true,
            Operator { op, args } => {
                op.arity() == args.len() && args.iter().all(|a| a.is_well_formed())
            }
        }
    }
}

impl<V: Ord, O> Term<V, O> {
    // Assign incrementing numbers to each variable encountered, returning the same number if the
    // same variable appears later. Traversal is depth-first.
    pub fn varseq(&self) -> Vec<u32> {
        let mut n: u32 = 0;
        let mut varmap: BTreeMap<&V, u32> = BTreeMap::new();
        let mut terms: Vec<&Term<V, O>> = vec![self];
        let mut vars: Vec<u32> = vec![];
        while let Some(t) = terms.pop() {
            match t {
                Variable { var } => {
                    vars.push(*varmap.entry(var).or_insert_with(|| {
                        // If the variable hasn't been seen yet, assign a new number to it.
                        n += 1;
                        n
                    }));
                }
                Operator { op: _, args } => {
                    for a in args.iter() {
                        terms.push(a);
                    }
                }
            }
        }
        vars
    }
}

impl<V: Ord, O: Eq + Arity> PartialEq for Term<V, O> {
    fn eq(&self, other: &Term<V, O>) -> bool {
        // First determine if the terms have the same operators.
        (self.is_well_formed() && other.is_well_formed())
            && (match self {
                Variable { var: _ } => other.is_var(),
                Operator { op: f, args: fargs } => match other {
                    Variable { var: _ } => false,
                    Operator { op: g, args: gargs } => {
                        f == g && fargs.iter().zip(gargs.iter()).all(|(fa, ga)| fa == ga)
                    }
                },
            })
            && self.varseq() == other.varseq()
    }
}

// TODO: are these the right trait bounds?
impl<V: Ord, O: Ord + Arity> PartialOrd for Term<V, O> {
    fn partial_cmp(&self, _other: &Term<V, O>) -> Option<Ordering> {
        // TODO
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fmt;

    #[derive(PartialEq, Eq, Debug)]
    enum Prod {
        Mul,
        Id,
    }
    use Prod::*;

    impl Arity for Prod {
        fn arity(&self) -> usize {
            match self {
                Mul => 2,
                Id => 0,
            }
        }
    }

    impl fmt::Display for Prod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Mul => write!(f, "*"),
                Id => write!(f, "1"),
            }
        }
    }

    type ProdTerm = Term<String, Prod>;

    #[test]
    fn well_formed() {
        let x = ProdTerm::var("x");
        let m = ProdTerm::op(Prod::Mul, vec![ProdTerm::var("x"), ProdTerm::var("y")]);
        let o = ProdTerm::op(Prod::Id, vec![]);
        let b = ProdTerm::op(Prod::Mul, vec![ProdTerm::var("x")]);
        assert!(x.is_well_formed());
        assert!(m.is_well_formed());
        assert!(o.is_well_formed());
        assert!(!b.is_well_formed());
    }

    #[test]
    fn display() {
        let m = ProdTerm::op(Prod::Mul, vec![ProdTerm::var("a"), ProdTerm::var("b")]);
        println!("{}", m);
    }

    #[test]
    fn eq() {
        let tab = ProdTerm::op(Prod::Mul, vec![ProdTerm::var("a"), ProdTerm::var("b")]);
        let tcd = ProdTerm::op(Prod::Mul, vec![ProdTerm::var("c"), ProdTerm::var("d")]);
        let taa = ProdTerm::op(Prod::Mul, vec![ProdTerm::var("a"), ProdTerm::var("a")]);
        let tbb = ProdTerm::op(Prod::Mul, vec![ProdTerm::var("b"), ProdTerm::var("b")]);
        assert_eq!(tab, tcd);
        assert_eq!(taa, tbb);
        assert!(!(taa == tab));
    }
}
