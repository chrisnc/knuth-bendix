use std::fmt::{self, Display};

use crate::{Operator, Term};

// TODO: allow subterms to have lifetimes shorter than the parent
#[derive(Clone, Debug)]
pub enum Prod<'a> {
    One,
    Mul {
        left: &'a Term<String, Prod<'a>>,
        right: &'a Term<String, Prod<'a>>,
    },
}
use Prod::*;

pub enum Iter<'a> {
    Empty,
    Right(&'a Term<String, Prod<'a>>),
    Both {
        left: &'a Term<String, Prod<'a>>,
        right: &'a Term<String, Prod<'a>>,
    },
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Term<String, Prod<'a>>;

    fn next(&mut self) -> Option<&'a Term<String, Prod<'a>>> {
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

impl<'a> Prod<'a> {
    pub fn mul(
        left: &'a Term<String, Prod<'a>>,
        right: &'a Term<String, Prod<'a>>,
    ) -> Term<String, Prod<'a>> {
        Term::Op(Mul { left, right })
    }

    pub fn var<S: Into<String>>(s: S) -> Term<String, Prod<'a>> {
        Term::Var(s.into())
    }

    pub fn one() -> Term<String, Prod<'a>> {
        Term::Op(One)
    }

    pub fn iter(&'a self) -> Iter {
        match self {
            One => Iter::Empty,
            Mul { left, right } => Iter::Both { left, right },
        }
    }
}

impl<'a> Operator<'a> for Prod<'a> {
    type Var = String;
    type ArgIter = Iter<'a>;

    fn arg_iter(&'a self) -> Self::ArgIter {
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
    t: &Term<V, Prod<'a>>,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    match t {
        Term::Var(v) => v.fmt(f),
        Term::Op(o) => o.fmt_with_parens(f),
    }
}

impl<'a> Prod<'a> {
    fn fmt_with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            One => "1".fmt(f),
            Mul { .. } => "(".fmt(f).and(self.fmt(f)).and(")".fmt(f)),
        }
    }
}

impl<'a> fmt::Display for Prod<'a> {
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
        let m = Prod::mul(&a, &b);
        let am = Prod::mul(&a, &m);
        println!("{}", m);
        println!("{}", am);
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
