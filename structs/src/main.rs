#[derive(Debug)]
struct R {
    width: u32,
    height: u32
}

impl R{
    fn area(&self)->u32{
    self.height * self.width}

    fn __eq__(&self, other: &R)->bool{
        if other.height == self.height && self.width == other.width{ 
            true
        }else{
            false
        }
        }
    }

    fn square (a:u32) -> R {

        R {
            height: a,
            width:a
        }
    }

    fn main() {

    // let black        =  Color(0, 0, 0);
    // let origin       =  Point(0, 0, 0);
    // let mut first    =  build_user(String::from("rtkushner"), String::from("rxz"));
    // let mut  second  =  User {
    //     email: String::from("rar"),
    //     username: String::from("caar"),
    //     ..first
    // };

    let rect1 = R {
        width   :  32,
        height  :  32
    };
    let rect2 = R {
        width   :  40,
        height  :  3
    };
    let rect3 = R {
        width   :  32,
        height  :  32
    };

    let width1   =  30;
    let height1  =  50;
    println!("Struct :{:#?} ", rect1);
    println!(" Struct 1 is equiv to struct 3 : {}",rect1.__eq__(&rect3));
    println!(" Struct 2 is equiv to struct 3 : {}",rect2.__eq__(&rect3));
}
