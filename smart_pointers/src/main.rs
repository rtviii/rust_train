use crate::List::{Cons,Nil};
use std::ops::Deref;
pub mod deref_coercion;
pub mod deref_two;
pub mod interior_mutability;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(r:T)->MyBox<T>{
        MyBox(r)
    }
}

impl <T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}


fn main() {

    // let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));

    let y = 15;
    // let s = &y;
    // let s = Box::new(y);
    let s = Box::new(y);

    assert_eq!(15,y);
    assert_eq!(15,*s);

    let m = MyBox::new(String::from("RUST"));
    deref_two::hello_again(&m);
    
    // ? ------------------- Drop : implementation and std::mem


    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("my other stuff"),
    };

    std::mem::drop(c);
    println!("Custom pointers created");

    // ? -----------------------------------RC

    let val = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&val),Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));



    *val.borrow_mut() += 10;
    println!("a aftr = {:?}", a);
    println!("b aftr = {:?}", b);
    println!("c aftr = {:?}", c);
}



    struct CustomSmartPointer{
        data:String,
    }


    impl Drop for CustomSmartPointer {
        fn drop (&mut self){
            println!("Dropping custom smart pointer with data `{}`!", self.data);
        }
    }