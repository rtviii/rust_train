


unsafe fn danger( )
{}



fn main() {

    unsafe{
        danger();
    }

    let mut num = 5;

    let r1 = &num as  *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe
    {
    println!("hey, unsafe world :{}", *r1);
    println!("hey, unsafe world2 :{}", *r2);
    println!("hey, unsafe world2 :{}", *r);
    }

}
