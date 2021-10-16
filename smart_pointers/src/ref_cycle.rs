use std::rc::Weak;
use crate::ref_cycle::RefCycleList::{Cons,Nil};
use std::rc::Rc;
use std::cell::RefCell;



#[derive(Debug)]
enum RefCycleList{
	Cons(i32, RefCell<Rc<RefCycleList>>),
	Nil,
}



impl RefCycleList {
	fn tail (&self)-> Option<&RefCell<Rc<RefCycleList>>>{
		match self {
			Cons(_, item) => Some(item),
			Nil => None,
		}
}}


pub fn main(){


    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//     Rc::downgrade(b);

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

}


#[derive(Debug)]
struct Node{
	value   : i32,
	parent  : RefCell<Weak<Node>>,
	children: RefCell<Vec<Rc<Node>>>,

}

pub fn tree(){

	let leaf = Rc::new(Node{
		value   : 3,
		parent  : RefCell:: new(Weak::new()),
		children: RefCell:: new(vec![])
	});


	println!(
		"leaf strong = {}, weak = {}",
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf)
	);

	{

		let branch = Rc::new(Node{
			value   : 5,
			parent  : RefCell:: new(Weak::new()),
			children: RefCell:: new(vec![Rc::clone(&leaf)])
		});


		*leaf.parent.borrow_mut() = Rc::downgrade(&branch);


		println!(
			"branch strong = {}, weak = {}",
			Rc::strong_count(&branch),
			Rc::weak_count(&branch),
		);

		println!(
			"leaf strong = {}, weak = {}",
			Rc::strong_count(&leaf),
			Rc::weak_count(&leaf)
		);

	}
	println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

		println!(
			"leaf strong = {}, weak = {}",
			Rc::strong_count(&leaf),
			Rc::weak_count(&leaf)
		);



}