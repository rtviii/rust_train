use std::fmt::Display;
use std::fmt::Debug;

pub fn notify(item : & impl Summary){
    println!("Breaking news, {}", item.summarize());
}

pub fn notify_desugared<T:Summary>(item : T){
    println!("Breaking news, {}", item.summarize());
}

pub fn notify_impl(item1: & impl Summary,item2:& impl Summary){}
pub fn notify_traitbound<T:Summary>(item1: &T,item2:&T){}
pub fn notify_multiple_tbounds<T:Summary + Display>(item1: &T){}

pub fn verbose<W,Y>(p1: &W, p2: &Y) ->i32 
where W: Display + Clone,
      Y: Clone + Debug
      {1}


use crate::generics::{ Tweet,Summary, NewsArticle};

pub fn returns_summarizable()-> impl Summary{
    Tweet{
        content:String::from("of course as yrahh"),
        username:String::from("horesy2424"),
        reply: false,
        retweet: true
    }
}



// pub fn returns_either()-> impl Summary{
//     if switch{
//         NewsArticle{

//         }
//     }
//     Tweet{
//         content:String::from("of course as yrahh"),
//         username:String::from("horesy2424"),
//         reply: false,
//         retweet: true
//     }
// }