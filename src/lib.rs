use rand::prelude::*;

mod adjectives;
mod animals;

use adjectives::ADJECTIVES;
use animals::ANIMALS;

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
/// The zoo can generate animals. Each animal can have between `0` and `u32::MAX` adjectives, and a
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
/// Generate a really fancy animal:
/// ```
/// use friendly_zoo::Zoo;
/// let animal = Zoo::new('$', 6).generate();
/// println!("{}", animal);
/// ```
pub struct Zoo {
    delimiter: char,
    // `u8` conveniently limits the number of adjectives to be in the same ballpark as the number
    // of adjectives actually available. This ensures no duplicate. Using more than 255 adjectives
    // is considered unreasonably excessive, anyway.
    number_of_adjectives: u8,
}

impl Zoo {
    pub fn new(delimiter: char, number_of_adjectives: u8) -> Self {
        Self {
            delimiter,
            number_of_adjectives,
        }
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let mut result = String::new();

        ADJECTIVES
            .choose_multiple(&mut rng, self.number_of_adjectives as usize)
            .for_each(|adjective| {
                result.push_str(adjective);
                result.push(self.delimiter);
            });

        result.push_str(ANIMALS.choose(&mut rng).unwrap());

        result
    }
}

impl Default for Zoo {
    fn default() -> Self {
        Self {
            delimiter: '_',
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
    fn different_delimiters() {
        for ch in "-+^#_".chars() {
            let animal = Zoo::new(ch, 5).generate();
            assert!(animal.chars().filter(|&c| c == ch).count() == 5);
        }
    }
}
