fn main() {
    let mut a = 3;
    let mut b = 5;
    println!("(a, b) = ({a:?}, {b:?})");
    core::mem::swap(&mut a, &mut b);
    println!("(a, b) = ({a:?}, {b:?})");
}
