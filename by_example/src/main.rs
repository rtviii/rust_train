struct Point(i32,i32);
struct Rectange {
    x: i32,
    y: i32
}

impl Rectange {
    fn area(&self)->i32{
        self.x * self.y
    }

    fn square(&self,p:Point,a:i32 ) ->Rectange{

        let Point(width,height) =p;

        println!("Desctructured a point with x={}, y={}", width, height);

        Rectange{ x:width+a, y:height+a }

    }
}



fn main() {

    let rec = Rectange{ x:2,y:4 };
    println!("Rect with widht {} and lenght {}", rec.x, rec.y);
    println!("corresponding area : {}", rec.area());
    
    let Rectange{
        x: square_width,
        y: square_height
    } = rec.square(Point(2,3), 4);


    println!("create a square and destructured {} x {}", square_width, square_height)



    

}
