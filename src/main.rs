mod words;

use clap::Parser;
use rand::Rng;
use words::{ADJECTIVES, NOUNS};

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

/// Picks one random adjective and one random noun.
///
/// The returned parts are in the original lowercase form from the word lists.
/// No formatting is applied.
pub fn generate_name(rng: &mut impl Rng) -> NameParts {
    let adjective = ADJECTIVES[rng.random_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rng.random_range(0..NOUNS.len())];
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
    for _ in 0..cli.number {
        let parts = generate_name(&mut rng);
        println!("{}", format_name(&parts, casing, &cli.delimiter));
    }
}

#[cfg(test)]
mod tests {
    use super::{Casing, NameParts, format_name, generate_name};
    use crate::words::{ADJECTIVES, NOUNS};
    use rand::{SeedableRng, rngs::SmallRng};
    use std::collections::HashSet;

    #[test]
    fn generate_name_returns_lowercase_adjective_and_noun() {
        let mut rng = SmallRng::seed_from_u64(42);
        let parts = generate_name(&mut rng);
        assert!(
            !parts.adjective.is_empty() && parts.adjective.chars().all(|c| c.is_ascii_lowercase())
        );
        assert!(!parts.noun.is_empty() && parts.noun.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn generate_name_picks_from_word_lists() {
        let mut rng = SmallRng::seed_from_u64(42);
        let parts = generate_name(&mut rng);
        assert!(ADJECTIVES.contains(&parts.adjective.as_str()));
        assert!(NOUNS.contains(&parts.noun.as_str()));
    }

    #[test]
    fn generate_name_is_deterministic_for_same_seed() {
        let a = generate_name(&mut SmallRng::seed_from_u64(42));
        let b = generate_name(&mut SmallRng::seed_from_u64(42));
        assert_eq!(a.adjective, b.adjective);
        assert_eq!(a.noun, b.noun);
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
    fn word_lists_contain_exactly_777_entries_each() {
        assert_eq!(ADJECTIVES.len(), 777);
        assert_eq!(NOUNS.len(), 777);
    }

    #[test]
    fn word_list_entries_are_non_empty_lowercase_ascii_alphabetic() {
        for word in ADJECTIVES {
            assert!(
                !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()),
                "adjective {word:?} contains non-lowercase-ASCII or is empty"
            );
        }
        for word in NOUNS {
            assert!(
                !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()),
                "noun {word:?} contains non-lowercase-ASCII or is empty"
            );
        }
    }

    #[test]
    fn word_lists_have_no_duplicates_and_no_cross_list_overlap() {
        let adj_set: HashSet<&str> = ADJECTIVES.iter().copied().collect();
        let noun_set: HashSet<&str> = NOUNS.iter().copied().collect();

        assert_eq!(
            adj_set.len(),
            ADJECTIVES.len(),
            "adjectives list contains duplicates"
        );
        assert_eq!(
            noun_set.len(),
            NOUNS.len(),
            "nouns list contains duplicates"
        );

        let overlap: Vec<&str> = adj_set.intersection(&noun_set).copied().collect();
        assert!(
            overlap.is_empty(),
            "words appear in both lists: {overlap:?}"
        );
    }
}
