use knuth_bendix::*;

fn main() {
    type T = Term::<String, String>;
    let t0 = T::op("+", vec![T::var("x"), T::var("y")]);
    let t1 = T::op("+", vec![T::var("a"), T::var("b")]);
    let t2 = T::op("+", vec![T::var("z"), T::var("z")]);
    println!("{:?}", t0.varseq());
    println!("{:?}", t1.varseq());
    println!("{:?}", t2.varseq());
}
