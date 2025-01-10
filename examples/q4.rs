fn main() {
    let mut a = 3;
    let mut b = 5;
    let a = &mut a;
    let b = &mut b;
    println!("(a, b) @ ({0:p}, {1:p}) = ({0:?}, {1:?})", a, b);
    core::mem::swap(a, b);
    println!("(a, b) @ ({0:p}, {1:p}) = ({0:?}, {1:?})", a, b);
}
