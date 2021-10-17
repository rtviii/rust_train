pub mod gui;
pub mod blogpost;
use gui::{Button, Screen, SelectBox};

pub struct AverageCollection{
    members: Vec<i32>,
    average: f64
}



impl AverageCollection {
    fn update_average(&mut self){
        let total:i32    = self.members.iter().sum();
            self.average = total as f64/ self.members.len() as f64;
    }
    pub fn add (&mut self, value: i32){
        self.members.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32>{ 

        match self.members.pop(){

            Some(value) =>{
                self.update_average();
                Some(value)
            }

            None => None
        }
     }

     pub fn average(&self) -> f64{
         self.average()
     }


}


fn main() {

    // let anotherScreen = Screen{
    //     components:vec![Box::new(String::from("Hi"))],
    // };


    let screen = Screen{
        components :vec![
            Box::new(SelectBox{
                width  : 75,
                height : 20,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ]
            }),
            Box::new(Button{
                width : 20,
                height: 5,
                label : String:: from("hey"),
            })
        ]
    };

    screen.run();

}
