use friendly_zoo::Zoo;

enum Arg {
    None,
    Delimiter,
    NumberOfAdjectives,
}

struct Args {
    delimiter: char,
    number_of_adjectives: u8,
}

fn main() {
    let mut args = Args {
        delimiter: '-',
        number_of_adjectives: 1,
    };

    let mut next_arg = Arg::None;

    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-d" | "--delimiter" => next_arg = Arg::Delimiter,
            "-n" | "--adjectives" => next_arg = Arg::NumberOfAdjectives,
            value => match next_arg {
                Arg::None => {
                    eprintln!("Invalid arguments!");
                    return;
                }
                Arg::Delimiter => {
                    args.delimiter = value.parse().unwrap();
                    next_arg = Arg::None;
                }
                Arg::NumberOfAdjectives => {
                    args.number_of_adjectives = value.parse().unwrap();
                    next_arg = Arg::None;
                }
            },
        }
    }

    println!(
        "{}",
        Zoo::new(args.delimiter, args.number_of_adjectives).generate()
    );
}
