use std::any::type_name_of_val;
use std::borrow::BorrowMut;
use std::marker::PhantomPinned;
use std::ops::Drop;
use std::pin::Pin;

#[derive(Debug)]
#[allow(unused)]
struct Wrapper(u8, PhantomPinned);
impl Drop for Wrapper {
    fn drop(&mut self) {
        println!("dropped: {self:?} @ {self:p}")
    }
}
impl Unpin for Wrapper {}

fn main() {
    type T = Wrapper;
    let mut a: Pin<Box<T>> = Box::pin(Wrapper(3u8, Default::default()));
    let mut b: Pin<Box<T>> = Box::pin(Wrapper(5u8, Default::default()));
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    core::mem::swap::<T>(a.borrow_mut(), b.borrow_mut());
    println!("(a, b) @ ({a:p}, {b:p}) = ({a:?}, {b:?})");
    println!("a: {}", type_name_of_val(&a));
    println!("b: {}", type_name_of_val(&b));
}
