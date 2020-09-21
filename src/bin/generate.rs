use friendly_zoo::{Species, Zoo};

enum Arg {
    None,
    Species,
    NumberOfAdjectives,
}

struct Args {
    species: Species,
    number_of_adjectives: u8,
}

fn parse_species(s: &str) -> Result<Species, &'static str> {
    match s.to_ascii_lowercase().as_ref() {
        "snake" => Ok(Species::Snake),
        "screaming_snake" => Ok(Species::ScreamingSnake),
        "camel" => Ok(Species::Camel),
        "dromedary" => Ok(Species::Dromedary),
        "kebab" => Ok(Species::Kebab),
        "screaming_kebab" => Ok(Species::ScreamingKebab),
        _ => Err("Not a valid species!"),
    }
}

fn main() {
    let mut args = Args {
        species: Species::Snake,
        number_of_adjectives: 1,
    };

    let mut next_arg = Arg::None;

    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-s" | "--species" => next_arg = Arg::Species,
            "-n" | "--adjectives" => next_arg = Arg::NumberOfAdjectives,
            value => match next_arg {
                Arg::None => {
                    eprintln!("Invalid arguments!");
                    return;
                }
                Arg::Species => {
                    args.species = parse_species(value).unwrap();
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
        Zoo::new(args.species, args.number_of_adjectives).generate()
    );
}
