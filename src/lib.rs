use rand::prelude::*;

mod adjectives;
mod animals;

use adjectives::ADJECTIVES;
use animals::ANIMALS;

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
}

/// What does the animal look like?
#[derive(Debug)]
pub enum Species {
    /// A `snake_like_animal`.
    Snake,

    /// A `VERY_LOUD_SNAKE_ANIMAL`.
    ScreamingSnake,

    /// A `CamelLikeAnimal`.
    Camel,

    /// A `dromedaryLikeAnimal`.
    Dromedary,

    /// Everyone knows the `kebab-animal`.
    Kebab,

    /// A `VERY-LOUD-KEBAB-ANIMAL`.
    ScreamingKebab,

    /// The rarest of species.
    CustomDelimiter(char),
}

impl Species {
    fn delimiter(&self) -> Option<char> {
        match self {
            Self::Snake | Self::ScreamingSnake => Some('_'),
            Self::Camel | Self::Dromedary => None,
            Self::Kebab | Self::ScreamingKebab => Some('-'),
            Self::CustomDelimiter(c) => Some(*c),
        }
    }
}

/// A friendly zoo.
///
/// The zoo can generate animals. Each animal can have between `0` and `u8::MAX` adjectives, and a
/// race. The number of adjectives and the delimiter between the adjectives and the animal are set
/// during instantiation. For one adjective, and `_` as the delimiter, use the `Default`
/// implementation of `Zoo`.
///
/// # Example
///
/// Just generate a friendly animal name:
/// ```
/// use friendly_zoo::Zoo;
/// let animal = Zoo::default().generate();
/// println!("{}", animal);
/// ```
///
/// Generate a `DifferentLooking` animal:
/// ```
/// use friendly_zoo::{Zoo, Species};
/// let animal = Zoo::new(Species::Camel, 6).generate();
/// println!("{}", animal);
/// ```
pub struct Zoo {
    species: Species,
    // `u8` conveniently limits the number of adjectives to be in the same ballpark as the number
    // of adjectives actually available. This ensures no duplicate. Using more than 255 adjectives
    // is considered unreasonably excessive, anyway.
    number_of_adjectives: u8,
}

impl Zoo {
    pub fn new(species: Species, number_of_adjectives: u8) -> Self {
        Self {
            species,
            number_of_adjectives,
        }
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let mut result = String::new();

        ADJECTIVES
            .choose_multiple(&mut rng, self.number_of_adjectives as usize)
            .enumerate()
            .for_each(|(i, adjective)| {
                match self.species {
                    Species::ScreamingSnake | Species::ScreamingKebab => {
                        result.push_str(&adjective.to_uppercase())
                    }
                    Species::Dromedary => match i {
                        0 => result.push_str(adjective),
                        _ => result.push_str(&capitalize(adjective)),
                    },
                    Species::Camel => result.push_str(&capitalize(adjective)),
                    _ => result.push_str(adjective),
                }
                if let Some(delimiter) = self.species.delimiter() {
                    result.push(delimiter);
                }
            });

        let animal = ANIMALS.choose(&mut rng).unwrap();
        match self.species {
            Species::ScreamingSnake | Species::ScreamingKebab => {
                result.push_str(&animal.to_uppercase())
            }
            Species::Dromedary => result.push_str(&capitalize(animal)),
            Species::Camel => result.push_str(&capitalize(animal)),
            _ => result.push_str(animal),
        }

        result
    }
}

impl Default for Zoo {
    fn default() -> Self {
        Self {
            species: Species::Snake,
            number_of_adjectives: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let animal = Zoo::default().generate();
        assert!(!animal.is_empty());
    }

    #[test]
    fn test_snakes() {
        let animal = Zoo::new(Species::Snake, 10).generate();
        assert_eq!(animal.chars().filter(|&c| c == '_').count(), 10);
        assert!(animal
            .chars()
            .filter(|&c| c != '_')
            .map(|c| c.is_ascii_lowercase())
            .all(|x| x));
    }

    #[test]
    fn test_screaming_snakes() {
        let animal = Zoo::new(Species::ScreamingSnake, 10).generate();
        assert_eq!(animal.chars().filter(|&c| c == '_').count(), 10);
        assert!(animal
            .chars()
            .filter(|&c| c != '_')
            .map(|c| c.is_ascii_uppercase())
            .all(|x| x));
    }

    #[test]
    fn test_dromedaries() {
        let animal = Zoo::new(Species::Dromedary, 10).generate();
        assert!(animal.chars().nth(0).unwrap().is_ascii_lowercase());
    }

    #[test]
    fn test_kebabs() {
        let animal = Zoo::new(Species::Kebab, 10).generate();
        assert_eq!(animal.chars().filter(|&c| c == '-').count(), 10);
        assert!(animal
            .chars()
            .filter(|&c| c != '-')
            .map(|c| c.is_ascii_lowercase())
            .all(|x| x));
    }

    #[test]
    fn test_screaming_kebabs() {
        let animal = Zoo::new(Species::ScreamingKebab, 10).generate();
        assert_eq!(animal.chars().filter(|&c| c == '-').count(), 10);
        assert!(animal
            .chars()
            .filter(|&c| c != '-')
            .map(|c| c.is_ascii_uppercase())
            .all(|x| x));
    }

    #[test]
    fn test_custom_delimiter() {
        let animal = Zoo::new(Species::CustomDelimiter('$'), 10).generate();
        assert_eq!(animal.chars().filter(|&c| c == '$').count(), 10);
        assert!(animal
            .chars()
            .filter(|&c| c != '$')
            .map(|c| c.is_ascii_lowercase())
            .all(|x| x));
    }
}
