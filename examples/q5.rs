fn main() {
    let mut a = 3;
    let mut b = 5;
    let a = &mut a;
    let b = &mut b;
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    core::mem::swap(a, b);
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
}
