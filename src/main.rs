use calculator::Rpn;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct Args {
    equation: String,
}

fn main() {
    let arg = Args::parse();
    let calc = Rpn::new();
    println!("{}", calc.solve(arg.equation).unwrap());
}
