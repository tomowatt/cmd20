use clap::{App, Arg};
use rand::Rng;

fn main() {
    let matches = App::new("cmd20")
        .version("0.1.0")
        .arg(
            Arg::with_name("dice")
                .short("d")
                .long("dice")
                .default_value("20")
                .possible_values(&["4", "6", "8", "10", "12", "20", "100"]),
        )
        .get_matches();

    let dice: usize = matches.value_of("dice").unwrap().parse().unwrap();

    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, dice + 1);

    //TODO: Add dice templates
    // let template = match dice {
    //     100 => r"",
    //     20 => r"",
    //     12 => r"",
    //     10 => r"",
    //     8 => r"",
    //     6 => r"",
    //     4 => r"",
    //     _ => "@",
    // };

    println!("{}", roll);
}
