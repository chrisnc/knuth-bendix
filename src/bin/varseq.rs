use knuth_bendix::prod::*;

fn main() {
    let a = Prod::var("a");
    let b = Prod::var("b");
    let x = Prod::var("x");
    let y = Prod::var("y");
    let z = Prod::var("z");
    let t0 = Prod::mul(&a, &b);
    let t1 = Prod::mul(&x, &y);
    let t2 = Prod::mul(&z, &z);
    let t3 = Prod::mul(&t0, &t2);
    println!("{}: {:?}", t0, t0.varseq());
    println!("{}: {:?}", t1, t1.varseq());
    println!("{}: {:?}", t2, t2.varseq());
    println!("{}: {:?}", t3, t3.varseq());
}
