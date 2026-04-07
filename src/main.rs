mod words;

use clap::Parser;
use rand::Rng;
use words::ADJECTIVES;

/// Noun category for name generation.
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum ThingCategory {
    /// Everyday objects (default).
    Objects,
    /// Fruits, vegetables, and other produce.
    Produce,
    /// Animals from around the world.
    Animals,
}

impl ThingCategory {
    /// Returns the noun word list for this category.
    pub fn nouns(&self) -> &'static [&'static str] {
        match self {
            Self::Objects => words::OBJECTS,
            Self::Produce => words::PRODUCE,
            Self::Animals => words::ANIMALS,
        }
    }
}

impl std::fmt::Display for ThingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Objects => write!(f, "objects"),
            Self::Produce => write!(f, "produce"),
            Self::Animals => write!(f, "animals"),
        }
    }
}

/// Generates a random name from a curated list of adjectives and nouns.
///
/// By default the output is ALL CAPS with no delimiter between words.
/// Use `--lower` for lowercase output and `--delimiter` to insert a separator.
/// Use `--number` to generate multiple names at once (up to 1000).
#[derive(Parser)]
struct Cli {
    /// Output the name in lowercase instead of the default ALL CAPS.
    #[arg(long)]
    lower: bool,

    /// String placed between words in the output. Defaults to no separator.
    #[arg(long, default_value = "")]
    delimiter: String,

    /// Number of names to generate (1-1000).
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=1000))]
    number: u32,

    /// Which noun category to draw from.
    #[arg(long, default_value_t = ThingCategory::Objects, value_enum)]
    things: ThingCategory,
}

/// The two components of a generated name: an adjective and a noun.
pub struct NameParts {
    pub adjective: String,
    pub noun: String,
}

/// Controls whether output is uppercased or lowercased.
#[derive(Clone, Copy)]
pub enum Casing {
    Upper,
    Lower,
}

/// Picks one random adjective and one random noun from the provided slice.
///
/// The returned parts are in the original lowercase form from the word lists.
/// No formatting is applied.
pub fn generate_name(rng: &mut impl Rng, nouns: &[&str]) -> NameParts {
    debug_assert!(!nouns.is_empty());
    let adjective = ADJECTIVES[rng.random_range(0..ADJECTIVES.len())];
    let noun = nouns[rng.random_range(0..nouns.len())];
    NameParts {
        adjective: adjective.to_string(),
        noun: noun.to_string(),
    }
}

/// Joins the name parts with `delimiter` and applies casing to the words only.
///
/// The delimiter is preserved as-is — only the adjective and noun are cased.
pub fn format_name(parts: &NameParts, casing: Casing, delimiter: &str) -> String {
    let (adj, noun) = match casing {
        Casing::Upper => (parts.adjective.to_uppercase(), parts.noun.to_uppercase()),
        Casing::Lower => (parts.adjective.to_lowercase(), parts.noun.to_lowercase()),
    };
    format!("{adj}{delimiter}{noun}")
}

fn main() {
    let cli = Cli::parse();
    let mut rng = rand::rng();
    let casing = if cli.lower {
        Casing::Lower
    } else {
        Casing::Upper
    };
    let nouns = cli.things.nouns();
    for _ in 0..cli.number {
        let parts = generate_name(&mut rng, nouns);
        println!("{}", format_name(&parts, casing, &cli.delimiter));
    }
}

#[cfg(test)]
mod tests {
    use super::{Casing, NameParts, ThingCategory, format_name, generate_name};
    use crate::words;
    use crate::words::{ADJECTIVES, ANIMALS, OBJECTS, PRODUCE};
    use rand::{SeedableRng, rngs::SmallRng};
    use std::collections::HashSet;

    #[test]
    fn generate_name_returns_lowercase_adjective_and_noun() {
        let mut rng = SmallRng::seed_from_u64(42);
        let parts = generate_name(&mut rng, words::OBJECTS);
        assert!(
            !parts.adjective.is_empty() && parts.adjective.chars().all(|c| c.is_ascii_lowercase())
        );
        assert!(!parts.noun.is_empty() && parts.noun.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn generate_name_picks_from_word_lists() {
        let mut rng = SmallRng::seed_from_u64(42);
        let parts = generate_name(&mut rng, words::OBJECTS);
        assert!(ADJECTIVES.contains(&parts.adjective.as_str()));
        assert!(words::OBJECTS.contains(&parts.noun.as_str()));
    }

    #[test]
    fn generate_name_is_deterministic_for_same_seed() {
        let a = generate_name(&mut SmallRng::seed_from_u64(42), words::OBJECTS);
        let b = generate_name(&mut SmallRng::seed_from_u64(42), words::OBJECTS);
        assert_eq!(a.adjective, b.adjective);
        assert_eq!(a.noun, b.noun);
    }

    #[test]
    fn thing_category_objects_returns_objects_list() {
        assert_eq!(ThingCategory::Objects.nouns(), words::OBJECTS);
    }

    #[test]
    fn thing_category_produce_returns_produce_list() {
        assert_eq!(ThingCategory::Produce.nouns(), words::PRODUCE);
    }

    #[test]
    fn thing_category_animals_returns_animals_list() {
        assert_eq!(ThingCategory::Animals.nouns(), words::ANIMALS);
    }

    fn parts(adj: &str, noun: &str) -> NameParts {
        NameParts {
            adjective: adj.to_string(),
            noun: noun.to_string(),
        }
    }

    #[test]
    fn format_name_upper_no_delimiter_produces_all_caps_concatenated() {
        assert_eq!(
            format_name(&parts("bold", "falcon"), Casing::Upper, ""),
            "BOLDFALCON"
        );
    }

    #[test]
    fn format_name_lower_no_delimiter_produces_lowercase_concatenated() {
        assert_eq!(
            format_name(&parts("bold", "falcon"), Casing::Lower, ""),
            "boldfalcon"
        );
    }

    #[test]
    fn format_name_upper_with_hyphen_delimiter() {
        assert_eq!(
            format_name(&parts("bold", "falcon"), Casing::Upper, "-"),
            "BOLD-FALCON"
        );
    }

    #[test]
    fn format_name_lower_with_hyphen_delimiter() {
        assert_eq!(
            format_name(&parts("bold", "falcon"), Casing::Lower, "-"),
            "bold-falcon"
        );
    }

    #[test]
    fn format_name_upper_with_underscore_delimiter() {
        assert_eq!(
            format_name(&parts("bold", "falcon"), Casing::Upper, "_"),
            "BOLD_FALCON"
        );
    }

    #[test]
    fn format_name_delimiter_is_not_cased() {
        assert_eq!(
            format_name(&parts("bold", "falcon"), Casing::Upper, "x"),
            "BOLDxFALCON"
        );
    }

    #[test]
    fn format_name_multi_character_delimiter_preserved() {
        assert_eq!(
            format_name(&parts("bold", "falcon"), Casing::Upper, "---"),
            "BOLD---FALCON"
        );
    }

    #[test]
    fn word_lists_fall_within_expected_size_ranges() {
        assert!(
            (300..=500).contains(&ADJECTIVES.len()),
            "ADJECTIVES has {} entries, expected 300–500",
            ADJECTIVES.len()
        );
        assert!(
            (200..=300).contains(&OBJECTS.len()),
            "OBJECTS has {} entries, expected 200–300",
            OBJECTS.len()
        );
        assert!(
            (150..=250).contains(&PRODUCE.len()),
            "PRODUCE has {} entries, expected 150–250",
            PRODUCE.len()
        );
        assert!(
            (200..=300).contains(&ANIMALS.len()),
            "ANIMALS has {} entries, expected 200–300",
            ANIMALS.len()
        );
    }

    #[test]
    fn word_list_entries_are_non_empty_lowercase_ascii_alphabetic() {
        for word in ADJECTIVES {
            assert!(
                !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()),
                "adjective {word:?} contains non-lowercase-ASCII or is empty"
            );
        }
        for word in OBJECTS {
            assert!(
                !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()),
                "objects entry {word:?} contains non-lowercase-ASCII or is empty"
            );
        }
        for word in PRODUCE {
            assert!(
                !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()),
                "produce entry {word:?} contains non-lowercase-ASCII or is empty"
            );
        }
        for word in ANIMALS {
            assert!(
                !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()),
                "animals entry {word:?} contains non-lowercase-ASCII or is empty"
            );
        }
    }

    #[test]
    fn word_lists_have_no_duplicates_and_no_cross_list_overlap() {
        let adj_set: HashSet<&str> = ADJECTIVES.iter().copied().collect();
        let obj_set: HashSet<&str> = OBJECTS.iter().copied().collect();
        let produce_set: HashSet<&str> = PRODUCE.iter().copied().collect();
        let animals_set: HashSet<&str> = ANIMALS.iter().copied().collect();

        assert_eq!(
            adj_set.len(),
            ADJECTIVES.len(),
            "ADJECTIVES list contains duplicates"
        );
        assert_eq!(
            obj_set.len(),
            OBJECTS.len(),
            "OBJECTS list contains duplicates"
        );
        assert_eq!(
            produce_set.len(),
            PRODUCE.len(),
            "PRODUCE list contains duplicates"
        );
        assert_eq!(
            animals_set.len(),
            ANIMALS.len(),
            "ANIMALS list contains duplicates"
        );

        let adj_obj_overlap: Vec<&str> = adj_set.intersection(&obj_set).copied().collect();
        assert!(
            adj_obj_overlap.is_empty(),
            "words appear in both ADJECTIVES and OBJECTS: {adj_obj_overlap:?}"
        );

        let adj_produce_overlap: Vec<&str> = adj_set.intersection(&produce_set).copied().collect();
        assert!(
            adj_produce_overlap.is_empty(),
            "words appear in both ADJECTIVES and PRODUCE: {adj_produce_overlap:?}"
        );

        let adj_animals_overlap: Vec<&str> = adj_set.intersection(&animals_set).copied().collect();
        assert!(
            adj_animals_overlap.is_empty(),
            "words appear in both ADJECTIVES and ANIMALS: {adj_animals_overlap:?}"
        );
    }

    #[test]
    fn noun_categories_have_no_cross_list_overlap() {
        let obj_set: HashSet<&str> = OBJECTS.iter().copied().collect();
        let produce_set: HashSet<&str> = PRODUCE.iter().copied().collect();
        let animals_set: HashSet<&str> = ANIMALS.iter().copied().collect();

        let obj_produce_overlap: Vec<&str> = obj_set.intersection(&produce_set).copied().collect();
        assert!(
            obj_produce_overlap.is_empty(),
            "words appear in both OBJECTS and PRODUCE: {obj_produce_overlap:?}"
        );

        let obj_animals_overlap: Vec<&str> = obj_set.intersection(&animals_set).copied().collect();
        assert!(
            obj_animals_overlap.is_empty(),
            "words appear in both OBJECTS and ANIMALS: {obj_animals_overlap:?}"
        );

        let produce_animals_overlap: Vec<&str> =
            produce_set.intersection(&animals_set).copied().collect();
        assert!(
            produce_animals_overlap.is_empty(),
            "words appear in both PRODUCE and ANIMALS: {produce_animals_overlap:?}"
        );
    }
}
