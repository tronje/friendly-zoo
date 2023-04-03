//! A friendly zoo! Use it to generate neat animal names.

#![deny(missing_debug_implementations, missing_docs)]

mod adjectives;
mod animals;

use adjectives::ADJECTIVES;
use animals::ANIMALS;
use rand::prelude::*;

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
///
/// `Zoo` implements `Iterator`:
/// ```
/// use friendly_zoo::Zoo;
/// let zoo = Zoo::default();
/// for animal in zoo.take(5) {
///     println!("{}", animal);
/// }
/// ```
#[derive(Debug)]
pub struct Zoo {
    species: Species,
    // `u8` conveniently limits the number of adjectives to be in the same ballpark as the number
    // of adjectives actually available. This ensures no duplicate. Using more than 255 adjectives
    // is considered unreasonably excessive, anyway.
    number_of_adjectives: u8,
}

impl Zoo {
    /// Create a new zoo to generate animal names.
    ///
    /// # Examples
    ///
    /// ```
    /// use friendly_zoo::{Zoo, Species};
    /// let zoo = Zoo::new(Species::Kebab, 3);
    /// println!("{}", zoo.generate());
    /// // prints e.g. poor-ballsy-elegant-camel
    /// ```
    pub fn new(species: Species, number_of_adjectives: u8) -> Self {
        Self {
            species,
            number_of_adjectives,
        }
    }

    /// Generate an animal name according to the specification `self` was constructed with.
    ///
    /// "Specification" here refers to the `Species` and the number of adjectives that were chosen.
    ///
    /// Uses the RNG returned by `rand::thread_rng()` to choose adjectives and animal name.
    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        self.generate_with_rng(&mut rng)
    }

    /// Generate an animal name using the provided random number generator.
    ///
    /// Equivalent to [`generate`], but allows you to pass in your own RNG instance. May be useful
    /// when you want to seed your generator and generate repeatable results.
    pub fn generate_with_rng<R>(&self, rng: &mut R) -> String
    where
        R: Rng,
    {
        let mut result = String::new();

        ADJECTIVES
            .choose_multiple(rng, self.number_of_adjectives as usize)
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

        let animal = ANIMALS.choose(rng).unwrap();
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

    /// Change the `Species` used for generating the animal names.
    pub fn set_species(&mut self, species: Species) {
        self.species = species;
    }

    /// Change the `Species` used for generating the animal names (builder-pattern style).
    ///
    /// # Examples
    ///
    /// ```
    /// # use friendly_zoo::{Species, Zoo};
    /// let zoo = Zoo::default().with_species(Species::Camel);
    /// ```
    pub fn with_species(mut self, species: Species) -> Self {
        self.species = species;
        self
    }

    /// Change the number of adjectives that precede a generated animal name.
    pub fn set_adjectives(&mut self, n: u8) {
        self.number_of_adjectives = n;
    }

    /// Change the number of adjectives that precede a generated animal name (builder-pattern style).
    ///
    /// # Examples
    ///
    /// ```
    /// # use friendly_zoo::Zoo;
    /// let zoo = Zoo::default().with_adjectives(3);
    /// ```
    pub fn with_adjectives(mut self, n: u8) -> Self {
        self.number_of_adjectives = n;
        self
    }
}

impl Default for Zoo {
    /// A default zoo uses snake case and one adjective for its animal names.
    fn default() -> Self {
        Self {
            species: Species::Snake,
            number_of_adjectives: 1,
        }
    }
}

impl Iterator for Zoo {
    type Item = String;

    /// Generate another animal name.
    ///
    /// `Zoo` is an infinite iterator. It always returns `Some(String)`, never `None`.
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
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
        assert!(animal.chars().next().unwrap().is_ascii_lowercase());
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
