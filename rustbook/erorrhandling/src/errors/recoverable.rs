use std::fs::File;
use std::io::{ ErrorKind,Read };
use std::io;

pub fn matches(n:i64)->i64{
    let f = File::open("yello.txt");
    let f = match f {
        Ok (file)  => file,
        Err(erorr) => match erorr.kind() {
            ErrorKind::NotFound => match File::create("yello.txt"){
                Ok  (fc) => fc,
                Err(e)   => panic!("Failed to create the file. E : {:?}", e),
            },
            _ =>{
                panic!("problem openning file. couldnt create. ");
            }
        }
    };
    69
}

pub fn results(){
    let f  = File::open("yello.txt").unwrap();
}

pub fn read_urname_from_file()->Result<String, io::Error>{
    let f = File::open("yello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(er)=> return Err(er),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e)=> Err(e),
    }
}

// Propagating errors | "?" is a shorthand for bubbling the error value up
pub fn read_from_file_propagate()->Result<String, io::Error>{
    let mut f = File::open("yello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

}

pub fn bettermatch(n:i64)->i64{
    let f = File::open("yello.txt")
    .unwrap_or_else(|error| {
        if error.kind()== ErrorKind::NotFound{
            File:: create("yello.txt")
            .unwrap_or_else(|error|{
                panic!("Problem creating a file :  {:?}", error);
            })
        } 
        else 
        {
            panic!("Problem opening file : {:?}", error);
        }

    });

    69
}