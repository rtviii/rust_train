pub mod imp1;

pub fn by_value(mut a: i32 ){
    a = 27;
	println!("a changed from {}", a);
}

pub fn by_value_slice(mut a: String )->String{
    String::from(&a[..])
}
pub fn by_ref_slice(mut a: &str )->&str{
    &a[..]
}

pub fn by_mut_ref(a: &mut i32){
	println!("a changed from {}", a);
	*a = 6;
}

pub fn by_ref(a:&i32)->i32{
	let inner_x = a;
	println!("{}",inner_x);
	*inner_x
}

