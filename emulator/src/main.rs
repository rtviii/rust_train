
use clap::{App, Arg};

fn main() {
    let matches = App::new("emulator").args(&[Arg::new("target")
        .short('t')
        .long("target")
        .takes_value(true)]);

    let args = matches.get_matches();
    let tgt: f32 = args.value_of_t("target").unwrap_or(42.42);

    println!("Target : {}\n", tgt);
    println!("Bytes : {:?}", tgt.to_le_bytes());
    println!("Bits : {:b}", tgt.to_bits());

    let bits = tgt.to_bits();

    println!("sign bit : {:b}", sign_bit(bits));
    println!("exp : {}", isolate_exp(bits));
}

fn sign_bit(bits: u32) -> u32 {
    bits >> 31
}

fn isolate_exp(bits: u32) -> u32 {
    let exponent = bits >> 23;
    let exponent = exponent & 0xff;
    let exponent = exponent - 127;
    exponent
}

fn isolate_mantissa(bits: u32)->f32{

    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = bits & mask;
        if one_at_bit_i !=0 {
            let i_        = i as f32;
            let weight    = 2_f32.powf(i_-23.0);
                mantissa += weight;

        }
    }
    mantissa
}
