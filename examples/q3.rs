fn main() {
    let mut a = 3;
    let mut b = 5;
    println!("(a, b) @ ({0:p}, {1:p}) = ({0:?}, {1:?})", &mut a, &mut b);
    core::mem::swap(&mut a, &mut b);
    println!("(a, b) @ ({0:p}, {1:p}) = ({0:?}, {1:?})", &mut a, &mut b);
}
