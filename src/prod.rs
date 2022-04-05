use std::fmt::{self, Display};

use crate::{Operator, Term};

// TODO: allow subterms to have lifetimes shorter than the parent
#[derive(Clone, Debug)]
pub enum Prod {
    One,
    Mul {
        left: Box<Term<String, Prod>>,
        right: Box<Term<String, Prod>>,
    },
}
use Prod::*;

pub enum Iter<'a> {
    Empty,
    Right(&'a Term<String, Prod>),
    Both {
        left: &'a Term<String, Prod>,
        right: &'a Term<String, Prod>,
    },
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Term<String, Prod>;

    fn next(&mut self) -> Option<&'a Term<String, Prod>> {
        match *self {
            Iter::Empty => None,
            Iter::Right(right) => {
                *self = Iter::Empty;
                Some(right)
            }
            Iter::Both { left, right } => {
                *self = Iter::Right(right);
                Some(left)
            }
        }
    }
}

impl Prod {
    pub fn mul(
        left: Term<String, Prod>,
        right: Term<String, Prod>,
    ) -> Term<String, Prod> {
        Term::Op(Mul { left: Box::new(left), right: Box::new(right) })
    }

    pub fn var<S: Into<String>>(s: S) -> Term<String, Prod> {
        Term::Var(s.into())
    }

    pub fn one() -> Term<String, Prod> {
        Term::Op(One)
    }

    pub fn iter(&self) -> Iter<'_> {
        match self {
            One => Iter::Empty,
            Mul { left, right } => Iter::Both { left, right },
        }
    }
}

impl Operator for Prod {
    type Var = String;
    type ArgIter<'a> = Iter<'a>;

    fn arg_iter(&self) -> Iter {
        match self {
            One => Iter::Empty,
            Mul { left, right } => Iter::Both { left, right },
        }
    }

    fn opeq(&self, other: &Self) -> bool {
        match (self, other) {
            (One, One) => true,
            (Mul { .. }, Mul { .. }) => true,
            _ => false,
        }
    }
}

fn fmt_with_parens<'a, V: fmt::Display>(
    t: &Term<V, Prod>,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    match t {
        Term::Var(v) => v.fmt(f),
        Term::Op(o) => o.fmt_with_parens(f),
    }
}

impl Prod {
    fn fmt_with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            One => "1".fmt(f),
            Mul { .. } => "(".fmt(f).and(self.fmt(f)).and(")".fmt(f)),
        }
    }
}

impl fmt::Display for Prod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            One => write!(f, "1"),
            Mul { left, right } => fmt_with_parens(left, f)
                .and(" * ".fmt(f))
                .and(fmt_with_parens(right, f)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prod::*;

    #[test]
    fn display() {
        let a = Prod::var("a");
        let b = Prod::var("b");
        let m = Prod::mul(a.clone(), b.clone());
        let am = Prod::mul(a.clone(), m.clone());
        println!("{}", m);
        println!("{}", am);
    }

    #[test]
    fn eq() {
        let a = Prod::var("a");
        let b = Prod::var("b");
        let c = Prod::var("c");
        let d = Prod::var("d");
        let tab = Prod::mul(a.clone(), b.clone());
        let tcd = Prod::mul(c.clone(), d.clone());
        let taa = Prod::mul(a.clone(), a.clone());
        let tbb = Prod::mul(b.clone(), b.clone());
        assert_eq!(tab, tcd);
        assert_eq!(taa, tbb);
        assert!(taa != tab);
    }
}
