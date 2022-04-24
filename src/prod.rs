use std::fmt::{self, Display};
use std::ops;

use crate::{Operator, Term};

// TODO: allow subterms to have lifetimes longer than the parent
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

    fn index(&self) -> u64 {
        match self {
            One => 1,
            Mul { .. } => 2,
        }
    }
}

impl ops::Mul for Term<String, Prod> {
    type Output = Term<String, Prod>;
    fn mul(self, rhs: Self) -> Self::Output {
        Term::Op(Mul {
            left: Box::new(self),
            right: Box::new(rhs),
        })
    }
}

fn fmt_with_parens<'a, V: fmt::Display>(t: &Term<V, Prod>, f: &mut fmt::Formatter) -> fmt::Result {
    match t {
        Term::Var(v) => v.fmt(f),
        Term::Op(o) => match o {
            One => "1".fmt(f),
            Mul { .. } => "(".fmt(f).and(o.fmt(f)).and(")".fmt(f)),
        },
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
    use std::cmp::*;

    #[test]
    fn display() {
        let a = Prod::var("a");
        let b = Prod::var("b");
        let m = a.clone() * b.clone();
        let am = a.clone() * m.clone();
        println!("{}", m);
        println!("{}", am);
    }

    #[test]
    fn eq() {
        let a = Prod::var("a");
        let b = Prod::var("b");
        let c = Prod::var("c");
        let d = Prod::var("d");
        let tab = a.clone() * b.clone();
        let tcd = c.clone() * d.clone();
        let taa = a.clone() * a.clone();
        let tbb = b.clone() * b.clone();
        assert_eq!(tab, tcd);
        assert_eq!(taa, tbb);
        assert!(taa != tab);
    }

    #[test]
    fn var_seq() {
        let a = Prod::var("a");
        let b = Prod::var("b");
        let x = Prod::var("x");
        let y = Prod::var("y");
        let z = Prod::var("z");
        let t0 = a * b;
        let t1 = x * y;
        let t2 = z.clone() * z;
        let t3 = t0.clone() * t2.clone();
        assert_eq!(t0.var_seq(), vec![1, 2]);
        assert_eq!(t1.var_seq(), vec![1, 2]);
        assert_eq!(t2.var_seq(), vec![1, 1]);
        assert_eq!(t3.var_seq(), vec![1, 2, 3, 3]);
    }

    #[test]
    fn cmp() {
        let one = Prod::one();
        let a = Prod::var("a");
        let b = Prod::var("b");
        let c = Prod::var("b");
        assert_eq!(one.cmp(&one), Ordering::Equal);
        assert_eq!(one.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&one), Ordering::Less);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(a.cmp(&b), Ordering::Equal);
        let ab = a.clone() * b.clone();
        let abc = ab.clone() * c.clone();
        assert_eq!(one.cmp(&ab), Ordering::Less);
        assert_eq!(ab.cmp(&abc), Ordering::Less);
        let aa = a.clone() * a.clone();
        assert_eq!(aa.cmp(&ab), Ordering::Less);
    }
}
