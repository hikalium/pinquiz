use std::any::type_name_of_val;
use std::borrow::BorrowMut;
use std::pin::pin;

fn main() {
    type T = Box<u8>;
    let a: T = Box::new(3u8);
    let b: T = Box::new(5u8);
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    let mut a = pin!(a);
    let mut b = pin!(b);
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    core::mem::swap::<T>(a.borrow_mut(), b.borrow_mut());
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
}
