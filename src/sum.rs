use std::fmt::{self, Display};
use std::ops;
use std::slice;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sum {
    Zero,
    Negate,
    Add,
}
use Sum::*;

impl Display for Sum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Zero => "0".fmt(f),
            Negate => "−".fmt(f),
            Add => " + ".fmt(f),
        }
    }
}


use crate::word::{self, Op, Var};

pub type Symbol = word::Symbol<String, Sum>;
pub type Word = word::Word<String, Sum>;

impl word::Operator for Sum {
    fn min_weight() -> u64 {
        1
    }

    fn arity(&self) -> usize {
        match self {
            Zero => 0,
            Negate => 1,
            Add => 2,
        }
    }

    fn weight(&self) -> u64 {
        match self {
            Zero => 1,
            Negate => 0,
            Add => 1,
        }
    }
}

impl ops::Add for &Word {
    type Output = Word;
    fn add(self, rhs: &Word) -> Word {
        Word::op(Add, &[self.clone(), rhs.clone()])
    }
}

impl ops::Add<Word> for &Word {
    type Output = Word;
    fn add(self, rhs: Word) -> Word {
        Word::op(Add, &[self.clone(), rhs])
    }
}

impl ops::Add<&Word> for Word {
    type Output = Word;
    fn add(self, rhs: &Word) -> Word {
        Word::op(Add, &[self, rhs.clone()])
    }
}

impl ops::Add for Word {
    type Output = Word;
    fn add(self, rhs: Word) -> Word {
        Word::op(Add, &[self, rhs])
    }
}

pub fn var<VF: Into<String>>(v: VF) -> Word {
    Word::var(v)
}

pub fn zero() -> Word {
    Word::op(Zero, &[])
}

pub fn negate(w: &Word) -> Word {
    Word::op(Negate, slice::from_ref(w))
}

fn fmt_with_parens(w: &Word, f: &mut fmt::Formatter) -> fmt::Result {
    match w.syms.first() {
        Some(Var(v)) => v.fmt(f),
        Some(Op(Zero)) => Zero.fmt(f),
        Some(Op(Negate)) => {
            if let Some(arg) = w.subwords().next() {
                Negate.fmt(f).and(fmt_with_parens(&arg, f))
            } else {
                fmt::Result::Err(fmt::Error::default())
            }
        }
        Some(Op(Add)) => {
            let mut sw = w.subwords();
            if let (Some(left), Some(right)) = (sw.next(), sw.next()) {
                "(".fmt(f)
                    .and(fmt_with_parens(&left, f))
                    .and(Add.fmt(f))
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
             * Format a top-level Add without surrounding parentheses; otherwise, call into
             * fmt_with_parens.
             */
            Some(Op(Add)) => {
                let mut sw = self.subwords();
                if let (Some(left), Some(right)) = (sw.next(), sw.next()) {
                    fmt_with_parens(&left, f)
                        .and(Add.fmt(f))
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
    use crate::print_subs;
    use crate::critical_term;
    use crate::sum::*;
    use std::cmp::*;
    use std::collections::BTreeMap;

    #[test]
    fn well_formed() {
        let a = var("a");
        let p = &a + zero();
        assert!(a.is_well_formed());
        assert!(p.is_well_formed());
    }

    #[test]
    fn subwords() {
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let d = var("d");
        let p = &a + &b + &c + &d + zero();

        for sw in p.subwords() {
            println!("{}", sw);
        }
    }

    #[test]
    fn display() {
        let a = var("a");
        let b = var("b");
        let ab = &a + &b;
        let aab = &a + &ab;
        let aab_left = &a + &a + &b;
        let negateaab = negate(&aab);
        let negatezero = negate(&zero());
        println!("{}", ab);
        println!("{}", aab);
        println!("{}", aab_left);
        println!("{}", negateaab);
        println!("{}", negatezero);
    }

    #[test]
    fn eq() {
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let d = var("d");
        let ab = &a + &b;
        let cd = &c + &d;
        let aa = &a + &a;
        let bb = &b + &b;
        assert!(ab != cd);
        assert!(aa != bb);
        assert!(aa != ab);
        assert_eq!(ab, ab);
    }

    #[test]
    fn partial_cmp() {
        let o = zero();
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let negatec = negate(&c);
        let negatenegatec = negate(&negate(&c));
        assert_eq!(o.partial_cmp(&o), Some(Ordering::Equal));
        assert_eq!(o.partial_cmp(&a), None);
        assert_eq!(a.partial_cmp(&o), None);
        assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
        assert_eq!(a.partial_cmp(&b), None);
        let ab = &a + &b;
        let abc = &ab + &c;
        assert_eq!(o.partial_cmp(&ab), Some(Ordering::Less));
        assert_eq!(ab.partial_cmp(&abc), Some(Ordering::Less));
        let aa = &a + &a;
        assert_eq!(aa.partial_cmp(&ab), None);
        assert_eq!(negatec.partial_cmp(&c), Some(Ordering::Greater));
        assert_eq!(negatec.partial_cmp(&negatenegatec), Some(Ordering::Less));
    }

    #[test]
    fn subst() {
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let bc = &b + &c;
        let vars = BTreeMap::<String, Word>::from([("a".to_string(), bc.clone())]);
        assert_eq!(a.subst(&vars), bc);
    }

    #[test]
    fn unify() {
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let bc = &b + &c;
        if let Some(subs) = a.unify(&bc) {
            print_subs(&subs);
        }
        //println!("{:?}", a.unify(&bc));
    }

    #[test]
    fn critical() {
        let t = negate(&var("x")) + var("x");
        let u = (var("x") + var("y")) + var("z");
        println!("t: {}", t);
        println!("u: {}", u);
        if let Some(ct) = critical_term(&t, &u) {
            println!("critical term: {}", ct);
        } else {
            println!("no critical term found");
        }
    }
}
