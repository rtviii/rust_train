
//* I want to explore borrowing and imports:
// -- references
// -- mutable/immutable borrows
// -- slices


pub mod others;
fn main() {

    let x = 1;
    let mut y = 5;

    others::by_ref(&x);
    others::by_mut_ref(&mut y);

	others::by_value(y);
    others::by_value(x);

    others::by_ref(&x);
    println!("end : {}",x);
}
