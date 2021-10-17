
pub struct Post{

	state: Option<Box<dyn State>>,
	content: String

}

impl Post{

	pub fn new () -> Post{
		Post {
			state:Some(Box::new(Draft{})),
			content:String::new(),
		}
	}

	pub fn add_text(&mut self, text: &str){
		self.content.push_str(text);
		// self.state.as_mut().unwrap().add_text(text);
	}

	pub fn content (&self) -> &str{
		self.state.as_ref().unwrap().content(self)
	}

	pub fn approve(&mut self){
		if let Some(s) =self.state.take(){
			self.state = Some(s.approve())
		}
	}

	pub fn request_review (&mut self){
		if let Some(s) = self.state.take(){
			self.state = Some(s.request_review())
		}
	}
}

// Now we can start seeing the advantages of the state pattern:
// the request_review method on Post is the same no matter its state value. 
// Each state is responsible for its own rules.

trait  State           {

	fn request_review(self:Box<Self>)->Box<dyn State>;
	fn approve       (self:Box<Self>)->Box<dyn State>;
	fn reject        (self:Box<Self>)->Box<dyn State>;
	fn content <'a> (&self, post: &'a Post)-> &'a str{
		""
	}

}

struct Draft         {}
struct PendingReview {
	approval_count: u32
}
struct Published     {}
// struct Approved{}

impl State for Draft {
	fn request_review(self: Box<Self>)->Box<dyn State>{
		Box::new(PendingReview {
			approval_count: 0
		})
	}
	fn approve(self: Box<Self>)->Box<dyn State>{
		self
	}
	fn reject(self: Box<Self>) ->Box<dyn State>{
		self
	}

}

impl State for PendingReview {



	fn request_review(self: Box<Self>) ->Box<dyn State>{
		self
	}
	fn approve(  mut self: Box<Self>)->Box<dyn State> {
		if self.approval_count == 2{
			Box::new(Published{})
		}else{
			self.approval_count= self.approval_count +1;
			self
		}
	}

	fn reject(self: Box<Self>) ->Box<dyn State>{
		Box::new(Draft{})
	}
}
impl State for Published {

	fn request_review(self: Box<Self>) ->Box<dyn State>{
		self
	}
	fn approve(self: Box<Self>)->Box<dyn State>{
		self
	}
	fn content<'a>(&self, post: &'a Post)  -> &'a str {
		&post.content
	}
	fn reject(self: Box<Self>) ->Box<dyn State>{
		Box::new(Draft{})
	}
}

// impl State for Approved {
// 	fn request_review(self: Box<Self>) ->Box<dyn State>{
// 		self
// 	}
// }


fn main (){

	let mut post = Post::new();

	post.add_text("I ate air.");
	assert_eq!("",post.content());

	post.request_review();
	assert_eq!("", post.content);

	post.approve();
	assert_eq!("I ate air.", post.content());
}