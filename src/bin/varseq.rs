use knuth_bendix::prod::*;

fn main() {
    let a = Prod::var("a");
    let b = Prod::var("b");
    let x = Prod::var("x");
    let y = Prod::var("y");
    let z = Prod::var("z");
    let t0 = Prod::mul(a, b);
    let t1 = Prod::mul(x, y);
    let t2 = Prod::mul(z.clone(), z);
    let t3 = Prod::mul(t0.clone(), t2.clone());
    println!("{}: {:?}", t0.clone(), t0.varseq());
    println!("{}: {:?}", t1.clone(), t1.varseq());
    println!("{}: {:?}", t2.clone(), t2.varseq());
    println!("{}: {:?}", t3.clone(), t3.varseq());
}
