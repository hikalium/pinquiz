use std::borrow::BorrowMut;
fn main() {
    type T = u8;
    let mut value_a: T = 3;
    let mut value_b: T = 5;
    let ref_a = &mut value_a;
    let ref_b = &mut value_b;
    println!();
    println!("ref_a: {}", std::any::type_name_of_val(&ref_a));
    println!("ref_b: {}", std::any::type_name_of_val(&ref_b));

    println!(
        "(ref_a, ref_b) @ ({0:p}, {1:p}) = ({0:?}, {1:?})",
        ref_a, ref_b
    );
    std::mem::swap::<T>(ref_a.borrow_mut(), ref_b.borrow_mut());
    println!("swap(ref_a, ref_b)");
    println!(
        "(ref_a, ref_b) @ ({0:p}, {1:p}) = ({0:?}, {1:?})",
        ref_a, ref_b
    );
}
