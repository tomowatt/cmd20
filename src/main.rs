use clap::Parser;

use cmd20::cmd20;

#[cfg(test)]
mod tests;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 20)]
    dice: u8,
}

fn main() {
    let args = Args::parse();

    let dice: usize = args.dice.into();

    println!("{}", cmd20(dice));
}
