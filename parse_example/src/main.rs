#![allow(dead_code)]
use std::error::Error;
use regex::{Regex, internal::Char};
use std::{ 
    convert::From,
    env::args,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use lazy_static::lazy_static;
use std::collections::HashMap;


fn tokenmap()-> HashMap<&'static str, Token>{
    let mut m = HashMap::new();
    m.insert("VAL", AminoAcid::VAL.into());
    m.insert("GLN", AminoAcid::GLN.into());
    m.insert("ARG", AminoAcid::ARG.into());
    m.insert("LEU", AminoAcid::LEU.into());
    m.insert("THR", AminoAcid::THR.into());
    m.insert("TYR", AminoAcid::TYR.into());
    m.insert("SER", AminoAcid::SER.into());let mut m = HashMap::new();
    m.insert("VAL", AminoAcid::VAL.into());
    m.insert("GLN", AminoAcid::GLN.into());
    m.insert("ARG", AminoAcid::ARG.into());
    m.insert("LEU", AminoAcid::LEU.into());
    m.insert("THR", AminoAcid::THR.into());
    m.insert("TYR", AminoAcid::TYR.into());
    m.insert("SER", AminoAcid::SER.into());
    m.insert("PRO", AminoAcid::PRO.into());
    m.insert("CYS", AminoAcid::CYS.into());
    m.insert("GLY", AminoAcid::GLY.into());
    m.insert("ALA", AminoAcid::ALA.into());
    m.insert("MET", AminoAcid::MET.into());
    m.insert("PRO", AminoAcid::PRO.into());
    m.insert("CYS", AminoAcid::CYS.into());
    m.insert("GLY", AminoAcid::GLY.into());
    m.insert("ALA", AminoAcid::ALA.into());
    m.insert("MET", AminoAcid::MET.into());
    m

}

#[derive(Debug)]
enum AminoAcid {
    VAL, GLN ,ARG,
    LEU, THR ,TYR,
    SER, PRO ,CYS,
    GLY, ALA ,MET
}


impl From<Coordinate> for Token  {
    fn from(coord: Coordinate) -> Self {
        Self::Coordinate(coord)
    }
}
impl From<AminoAcid> for Token  {
    fn from(aa: AminoAcid) -> Self {
        Self::AminoAcid(aa)
    }
}


#[derive(Debug)]
struct Coordinate{
    x:f64,    y:f64,    z:f64
}


#[derive(Debug)]
enum Token {
    Coordinate(Coordinate),
    AminoAcid(AminoAcid),
}

// fn istoken(t:&str)->bool{
    

//     true
// }


// Want a function that returns a list of legal tokens given a &line.
fn process_line(line:&str)->Vec<Token>{
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    
    dbg!(tokens);
    return vec![AminoAcid::ALA.into()];
}


fn main()->Result<(),Box<dyn Error>> {
    let path       = args().nth(1).expect("no pattern provided");
    let newpath    = Path::new(&path);
    let file       = File::open(newpath).expect("no such file");
    let lines      = BufReader::new(file).lines();
    let mut tokens = Vec::new();
    for line in lines{
        match line{
            Ok(s) => { 
                tokens.append(&mut process_line(&s[..]))
            },
            Err(_) =>  continue
        }
    }

    // dbg!(tokens);
    Ok(())

}
