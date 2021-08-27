use clap::{App, Arg};
use rand::Rng;

fn pad(roll: usize, dice: usize) -> String {
    fn add_padding(roll: String) -> String {
        let padded: String = match roll.len() {
            3 => roll,
            2 => "@ *".replace("@", &roll[..1]).replace("*", &roll[1..]),
            1 => " @ ".replace("@", &roll),
            _ => roll,
        };
        padded
    };

    let padded: String = match dice {
        100 => add_padding(roll.to_string()),
        10..=99 => add_padding(roll.to_string()),
        _ => roll.to_string(),
    };
    padded
}

fn main() {
    let matches = App::new("cmd20")
        .version("0.7.0")
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

    let padded_roll = pad(roll, dice);

    //TODO: Add dice templates
    let template = match dice {
        100 => "    ___\n  /     \\\n /       \\\n \\  @  /\n  \\     /\n    ---",
        20 => "      . \n ¸·___|___`.\n|    / \\    |\n|\\  /@\\  /|\n|_\\/_____\\/_|\n `. \\   / .'\n   ` \\ / '",
        12 => "  ,'.\n,'   `.\n\\ @ /\n \\___/",
        10 => "  // \\\\\n //   \\\\\n// @ \\\\\n|/ \\ / \\|\n \\  |  /\n  \\ | /",
        8 => "  / \\\n / @ \\\n/_____\\\n\\     /\n \\   /\n  \\ /",
        6 => " -----\n|     |\n|  @  |\n|     |\n -----",
        4 => "  / \\\n / @ \\\n/_____\\",
        _ => "@",
    };

    println!("{}", template.replace("@", &padded_roll));
}
