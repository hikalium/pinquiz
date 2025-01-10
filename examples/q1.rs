fn main() {
    let a = 3;
    let b = 5;
    println!("(a, b) = ({a:?}, {b:?})");
    core::mem::swap(&mut a, &mut b);
    println!("(a, b) = ({a:?}, {b:?})");
}
