use std::fmt;

use crate::{Operator, Term};

#[derive(Clone, Debug)]
pub enum Prod<'a> {
    One,
    Mul {
        left: &'a Term<String, Prod<'a>>,
        right: &'a Term<String, Prod<'a>>,
    },
}
use Prod::*;

#[derive(PartialEq, Eq)]
pub enum Op {
    One,
    Mul,
}

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

    // TODO: use this
    fn fmt_with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            One => write!(f, "1"),
            Mul { left, right } => write!(f, "(")
                .and(write!(f, "{}", left))
                .and(write!(f, " * "))
                .and(write!(f, "{}", right))
                .and(write!(f, ")")),
        }
    }
}

impl<'a> Operator<'a> for Prod<'a> {
    type Var = String;
    type Op = Op;
    type Args = Iter<'a>;

    fn args(&'a self) -> Self::Args {
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

impl<'a> fmt::Display for Prod<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            One => write!(f, "1"),
            Mul { left, right } => left.fmt(f).and(write!(f, " * ")).and(right.fmt(f)),
        }

        // TODO: use parentheses for subexpressions
        /*
        let mut terms: VecDeque<&'a Self> = VecDeque::new();
        terms.push_back(self);
        while let Some(t) = terms.pop_front() {
            match p {
                One => {
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
        */
    }
}
