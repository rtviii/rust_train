#![allow(dead_code)]
use clap::{App, Arg};
use std::fmt::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec;

#[derive(Debug)]
struct Coordinate (f32);

#[derive(Debug)]
enum Token {
    Coordinate(Coordinate),
    AminoAcid(AminoAcid),
}

#[derive(Debug)]
enum AminoAcid {
    VAL,
    GLN,
    ARG,
    ASN,
    LEU,
    THR,
    LYS,
    TYR,
    SER,
    PRO,
    CYS,
    GLU,
    PHE,
    HIS,
    ASP,
    ILE,
    GLY,
    ALA,
    MET,
}

fn match_string(tgt: &str) -> Option<Token> {
    if let Ok(something) = tgt.parse::<f32>(){
        return Some(Coordinate(something).into())
    }
    match tgt {
        "ARG" => Some(AminoAcid::ARG.into()),
        "ASN" => Some(AminoAcid::ASN.into()),
        "VAL" => Some(AminoAcid::VAL.into()),
        "GLN" => Some(AminoAcid::GLN.into()),
        "ILE" => Some(AminoAcid::ILE.into()),
        "HIS" => Some(AminoAcid::HIS.into()),
        "PHE" => Some(AminoAcid::PHE.into()),
        "GLU" => Some(AminoAcid::GLU.into()),
        "LEU" => Some(AminoAcid::LEU.into()),
        "THR" => Some(AminoAcid::THR.into()),
        "LYS" => Some(AminoAcid::LYS.into()),
        "TYR" => Some(AminoAcid::TYR.into()),
        "SER" => Some(AminoAcid::SER.into()),
        "PRO" => Some(AminoAcid::PRO.into()),
        "CYS" => Some(AminoAcid::CYS.into()),
        "GLY" => Some(AminoAcid::GLY.into()),
        "MET" => Some(AminoAcid::MET.into()),
        
        _ => None,
    }
}

impl TryFrom<String> for Token {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let x = match match_string(&value) {
            Some(uu) => Ok(uu),
            None => Err(()),
        };
        x
    }
}

fn process_line(line: &str) -> Vec<Token> {
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let mut results: Vec<Token> = vec![];
    for template_string in tokens {
        match template_string.to_string().try_into() {
            Ok(token) => {
                println!("Parsed line to Token : {:?}", token);
                results.push(token)
            }
            _ => continue,
        };
    }
    return results;
}

impl From<Coordinate> for Token {
    fn from(coord: Coordinate) -> Self {
        Self::Coordinate(coord)
    }
}
impl From<AminoAcid> for Token {
    fn from(aa: AminoAcid) -> Self {
        Self::AminoAcid(aa)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = App::new("parser")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true),
        )
        .get_matches();

    let to_parse: &str = match args.value_of("file") {
        Some(fpath) => fpath,
        None => panic!("Provide path"),
    };

    let file                     = File::open(to_parse)?;
    let reader                   = BufReader::new(file);
    let mut alltokens:Vec<Token> = vec![];
    for line in reader.lines(){
        alltokens.append(&mut process_line(&line.unwrap_or(" ".to_string())))
    };

    dbg!(alltokens.len());
    Ok(())
}
