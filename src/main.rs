use clap::{App, Arg};

use cmd20::cmd20;

fn main() {
    let matches = App::new("cmd20")
        .version("0.8.0")
        .arg(
            Arg::with_name("dice")
                .short("d")
                .long("dice")
                .default_value("20")
                .possible_values(&["4", "6", "8", "10", "12", "20", "100"]),
        )
        .get_matches();

    let dice: usize = matches.value_of("dice").unwrap().parse().unwrap();

    println!("{}", cmd20(dice));
}
