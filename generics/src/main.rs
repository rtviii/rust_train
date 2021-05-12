#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
pub mod generics;
pub mod traitparameters;
pub mod conditional_impls;
pub mod lifetimes;

use generics::noon;

fn largest_i32(list : &[i32]) -> i32{
    let mut largest = list[0];

    for &item in list {
        if item >largest{
            largest = item;
        }
    }
    largest
}


fn largest_char(list : &[char]) -> char{
    let mut largest = list[0];

    for &item in list {
        if item >largest{
            largest = item;
        }
    }
    largest
}


fn largest <T: PartialOrd >(listoft: &[T])-> &T{
    let mut largest = &listoft[0];
    for i in listoft {
        if i > largest {
            largest = &i;
        }
    }
    largest
}

struct Point <T>{
    y: T,
    x: T
}
impl <T> Point <T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// Listing 10-10: An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
impl Point<i32>{
    fn sum  (&self)-> i32{
         self.x + self.y 
    }
}
struct DoublyPt<U,W>{
    x: U,
    y: W
}

// This is being implemented **on** a struct with 2 typeargs, but takes **another** generic as a parameter
// returns a mix of the two
impl <U,W> DoublyPt <U,W> {
    fn mixup <Z> (self, other: Point<Z>) -> DoublyPt<U,Z> {
        DoublyPt{
            x: self.x,
            y: other.y
        }
    }
    fn x (&self) -> &U{
        &self.x
    }
    fn y (&self) -> &W{
        &self.y
    }
}


fn main() {
    // let p  = Point    { x    : 5, y: 10 };
    // let tt = DoublyPt { x    : 4, y :2}  ;
    // let r  = tt      .  mixup(    p)     ;

    // println!("r.x = {}", r.x());
    // println!("r.y= {}", r.y());


    // traitparameters::returns_summarizable();

    lifetimes::driver();
}
