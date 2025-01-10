use std::any::type_name_of_val;
use std::marker::PhantomPinned;
use std::pin::pin;

fn main() {
    type T = (u8, PhantomPinned);
    let a: T = (3u8, Default::default());
    let b: T = (5u8, Default::default());
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    let a = pin!(a);
    let b = pin!(b);
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    //core::mem::swap::<T>(a.borrow_mut(), b.borrow_mut());
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
}
