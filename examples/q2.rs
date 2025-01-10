use std::borrow::BorrowMut;
fn main() {
    type T = u8;
    let mut a: T = 3;
    let mut b: T = 5;
    let a = &mut a;
    let b = &mut b;
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})",);
    std::mem::swap::<T>(a.borrow_mut(), b.borrow_mut());
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})",);
}
