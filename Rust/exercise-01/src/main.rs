use std::error::Error;
use std::fmt;
use clap::Parser;

#[derive(Clone, Debug)]
struct NonPositiveError {
    message: String
}
impl NonPositiveError {
    fn new(msg: &str) -> Self {
        Self { message: msg.to_string() }
    }
}
impl fmt::Display for NonPositiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}
impl Error for NonPositiveError {}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, required = true)]
    bill: f64,
    
    #[arg(long, required = true)]
    tip_percent: u64,
}

fn validate_bill(bill: f64) -> Result<(), NonPositiveError> { 
    if bill <= 0.0 {
        Err(NonPositiveError::new("The bill cannot be a nonpositive number"))
    } else {
        Ok(())
    }
}

fn validate_args(args: &Args) {
    if let Err(e) = validate_bill(args.bill) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn calculate_tip(args: Args) -> f64 {
    let tip_multiplier = 100.0 + args.tip_percent as f64;
    tip_multiplier * args.bill / 100.0
}

fn main() {
    let args = Args::parse();
    validate_args(&args);
    println!("The total comes out to {}", calculate_tip(args));
}
