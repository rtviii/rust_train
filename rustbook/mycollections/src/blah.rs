fn main() {

    let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();

    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);
    let s = vec![1,2,3,4,5,6,7];


    let third: &i32 = &s[2];
    println!(" the third elem is {}", third);



    match s.get(5) {
        Some(third)=> println!("the third elem is {}", third),
        None=> println!("There is no third")
    }

    for i in &s {
        println!("{}",i);
    }

    enum Figures{
        Pawn(i32),
        Queen(i64),
        King(i128)
    }


let row = vec![
    Figures::Pawn(2),
    Figures::Pawn(2),
    Figures::Pawn(3),
    Figures::Pawn(2),
    Figures::Queen(2123123i64),
    Figures::King(-2123123i128),
];

}


