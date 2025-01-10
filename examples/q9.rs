use std::any::type_name_of_val;
use std::borrow::BorrowMut;
use std::pin::Pin;

fn main() {
    let mut a = 3u8;
    let mut b = 5u8;
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    let a = &mut a;
    let b = &mut b;
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    let a = Pin::new(a);
    let b = Pin::new(b);
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    core::mem::swap(a.borrow_mut(), b.borrow_mut());
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
}
