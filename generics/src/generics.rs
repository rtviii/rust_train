
pub trait Summary{
    fn summarize (&self) ->String{
        // default implementation
        String::from("Read more")
    }
    // another interface to implement
    fn createSummary (&self) -> ();
}

pub fn noon(){
    println!("Summarizing different texts: ");
    let tweet = Tweet {
        username : String:: from("horse_ebooks")                                  ,
        content  : String:: from("of course, as you probably already know, people",),
        reply    : false                                                          ,
        retweet  : false                                                          ,
    };

    println!("1 new tweet: {}", tweet.summarize());
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author  : String,
    pub content : String
}


impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{}, by {} ({})", self.headline, self.author,self.location)
    }
    fn createSummary(&self){
    }
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{} : {}", self.content, self.username)
    }
    fn createSummary(&self){
    }
}


pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool
}