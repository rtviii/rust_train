


#[derive(Debug)]
struct Rectangle{
    w: i32,
    h: i32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle)-> bool{
        self.w > other.w && self.h > other.h
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn explor() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn panicky(){
        panic!("this fails");
    }
}
