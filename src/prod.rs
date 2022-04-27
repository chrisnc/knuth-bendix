use std::fmt::{self, Display};
use std::ops;
use std::slice;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Prod {
    One,
    Inv,
    Mul,
}
use Prod::*;

impl Display for Prod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            One => "1".fmt(f),
            Inv => "⁻¹".fmt(f),
            Mul => " * ".fmt(f),
        }
    }
}


use crate::word::{self, Op, Var};

pub type Symbol = word::Symbol<String, Prod>;
pub type Word = word::Word<String, Prod>;

impl word::Operator for Prod {
    fn min_weight() -> u64 {
        1
    }

    fn arity(&self) -> usize {
        match self {
            One => 0,
            Inv => 1,
            Mul => 2,
        }
    }

    fn weight(&self) -> u64 {
        match self {
            One => 1,
            Inv => 0,
            Mul => 1,
        }
    }

    fn op_index(&self) -> u64 {
        match self {
            One => 0,
            Inv => 1,
            Mul => 2,
        }
    }
}

impl ops::Mul for &Word {
    type Output = Word;
    fn mul(self, rhs: &Word) -> Word {
        Word::op(Mul, &[self.clone(), rhs.clone()])
    }
}

impl ops::Mul<Word> for &Word {
    type Output = Word;
    fn mul(self, rhs: Word) -> Word {
        Word::op(Mul, &[self.clone(), rhs])
    }
}

impl ops::Mul<&Word> for Word {
    type Output = Word;
    fn mul(self, rhs: &Word) -> Word {
        Word::op(Mul, &[self, rhs.clone()])
    }
}

impl ops::Mul for Word {
    type Output = Word;
    fn mul(self, rhs: Word) -> Word {
        Word::op(Mul, &[self, rhs])
    }
}

pub fn var<VF: Into<String>>(v: VF) -> Word {
    Word::var(v)
}

pub fn one() -> Word {
    Word::op(One, &[])
}

pub fn inv(w: &Word) -> Word {
    Word::op(Inv, slice::from_ref(w))
}

fn fmt_with_parens(w: &Word, f: &mut fmt::Formatter) -> fmt::Result {
    match w.syms.first() {
        Some(Var(v)) => v.fmt(f),
        Some(Op(One)) => One.fmt(f),
        Some(Op(Inv)) => {
            if let Some(arg) = w.subwords().next() {
                fmt_with_parens(&arg, f).and(Inv.fmt(f))
            } else {
                fmt::Result::Err(fmt::Error::default())
            }
        }
        Some(Op(Mul)) => {
            let mut sw = w.subwords();
            if let (Some(left), Some(right)) = (sw.next(), sw.next()) {
                "(".fmt(f)
                    .and(fmt_with_parens(&left, f))
                    .and(Mul.fmt(f))
                    .and(fmt_with_parens(&right, f))
                    .and(")".fmt(f))
            } else {
                fmt::Result::Err(fmt::Error::default())
            }
        }
        None => fmt::Result::Err(fmt::Error::default()),
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.syms.first() {
            /*
             * Format a top-level Mul without surrounding parentheses; otherwise, call into
             * fmt_with_parens.
             */
            Some(Op(Mul)) => {
                let mut sw = self.subwords();
                if let (Some(left), Some(right)) = (sw.next(), sw.next()) {
                    fmt_with_parens(&left, f)
                        .and(Mul.fmt(f))
                        .and(fmt_with_parens(&right, f))
                } else {
                    fmt::Result::Err(fmt::Error::default())
                }
            },
            _ => fmt_with_parens(self, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prod::*;
    use std::cmp::*;

    #[test]
    fn well_formed() {
        let a = var("a");
        let p = &a * one();
        assert!(a.is_well_formed());
        assert!(p.is_well_formed());
    }

    #[test]
    fn subwords() {
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let d = var("d");
        let p = &a * &b * &c * &d * one();

        for sw in p.subwords() {
            println!("{}", sw);
        }
    }

    #[test]
    fn display() {
        let a = var("a");
        let b = var("b");
        let ab = &a * &b;
        let aab = &a * &ab;
        let aab_left = &a * &a * &b;
        let invaab = inv(&aab);
        let invone = inv(&one());
        println!("{}", ab);
        println!("{}", aab);
        println!("{}", aab_left);
        println!("{}", invaab);
        println!("{}", invone);
    }

    #[test]
    fn eq() {
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let d = var("d");
        let ab = &a * &b;
        let cd = &c * &d;
        let aa = &a * &a;
        let bb = &b * &b;
        assert!(ab != cd);
        assert!(aa != bb);
        assert!(aa != ab);
        assert_eq!(ab, ab);
    }

    #[test]
    fn partial_cmp() {
        let o = one();
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let invc = inv(&c);
        let invinvc = inv(&inv(&c));
        assert_eq!(o.partial_cmp(&o), Some(Ordering::Equal));
        assert_eq!(o.partial_cmp(&a), None);
        assert_eq!(a.partial_cmp(&o), None);
        assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
        assert_eq!(a.partial_cmp(&b), None);
        let ab = &a * &b;
        let abc = &ab * &c;
        assert_eq!(o.partial_cmp(&ab), Some(Ordering::Less));
        assert_eq!(ab.partial_cmp(&abc), Some(Ordering::Less));
        let aa = &a * &a;
        assert_eq!(aa.partial_cmp(&ab), None);
        assert_eq!(invc.partial_cmp(&c), Some(Ordering::Greater));
        assert_eq!(invc.partial_cmp(&invinvc), Some(Ordering::Less));
    }
}
