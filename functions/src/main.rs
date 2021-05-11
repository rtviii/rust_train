
fn main() {
    println!("Hello, world!");


    let y = {another_function( { 2 }-{1} )};

    println!(" y is therefore {}", y);


    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10{
            break counter *2;
        };
    };
    println!("counter : {}", result);




    let a = [10;5];
    for element in a.iter(){
        println!("value is {}",element)
    }
    // println!("A :{}", a[0])


    for k in  ( 2..a[0] ).rev()  {
        println!("k + 1 is {}", {k+1})
    }
}



fn another_function(x:u128)->usize{

    println!("yup {}",x);

    return if x < 2 {4} else {24};

}