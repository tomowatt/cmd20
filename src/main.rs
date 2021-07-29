use clap::{App, Arg};
use rand::Rng;

fn main() {
    let matches = App::new("cmd20")
        .version("0.3.0")
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
    let template = match dice {
        100 => "@",
        20 => "@",
        12 => "@",
        10 => "  // \\\\\n //   \\\\\n// @ \\\\\n|/ \\ / \\|\n \\  |  /\n  \\ | /",
        8 => "  / \\\n / @ \\\n/_____\\\n\\     /\n \\   /\n  \\ /",
        6 => " -----\n|     |\n|  @  |\n|     |\n -----",
        4 => "  / \\\n / @ \\\n/_____\\",
        _ => "@",
    };

    println!("{}", template.replace("@", &roll.to_string()));
}
