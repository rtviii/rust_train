use std::fmt::Result;

use args::{self, Args, ArgsError};
use getopts::Occur;

fn main() {
    // match parse
}

fn parse(input: &Vec<&str>) -> Result<(), ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);

    args.flag("h", "help", "Print the usage menu");

    args.option("i", "itern", "number of iterations", "times", Occur::Req, None);
    args.parse(input)?;

    let help = args.values_of("help");
    if help{
        args.full_usage();
        return Ok(());
    }
}
