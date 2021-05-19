use crate::List::{Cons,Nil};
use std::ops::Deref;
pub mod deref_coercion;

enum List{
    Cons(i32, Box<List>),
    Nil,
}




pub struct MyBox<T>(T);

impl <T>MyBox<T>{
    pub fn new(x:T)-> MyBox<T>{
        MyBox(x)
    }
}

impl <T> Deref for  MyBox<T>{
    type Target = T;
    fn deref(&self)->&T{
        &self.0
    }
}

fn main() {

    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));

    let x    = 5;
    let y    = &x;
    let z    = Box::new(x);
    let k    = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);
    assert_eq!(5,*z);
    assert_eq!(5,*k);
}

