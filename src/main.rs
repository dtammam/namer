use clap::Parser;
use rand::Rng;

/// Generates a random name from a curated list of adjectives and nouns.
///
/// By default the output is ALL CAPS with no delimiter between words.
/// Use `--lower` for lowercase output and `--delimiter` to insert a separator.
#[derive(Parser)]
struct Cli {
    /// Output the name in lowercase instead of the default ALL CAPS.
    #[arg(long)]
    lower: bool,

    /// String placed between words in the output. Defaults to no separator.
    #[arg(long)]
    delimiter: Option<String>,
}

const ADJECTIVES: &[&str] = &[
    "bold", "bright", "calm", "eager", "fierce", "gentle", "happy", "keen", "lively", "noble",
];

const NOUNS: &[&str] = &[
    "cloud", "falcon", "grove", "harbor", "island", "jaguar", "koala", "lantern", "meadow", "river",
];

/// Picks one random adjective and one random noun and returns them as a `Vec<String>`.
///
/// The returned vec always has exactly two elements: `[adjective, noun]`, both
/// in the original lowercase form from the word lists. No formatting is applied.
pub fn generate_name(rng: &mut impl Rng) -> Vec<String> {
    let adjective = ADJECTIVES[rng.random_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rng.random_range(0..NOUNS.len())];
    vec![adjective.to_string(), noun.to_string()]
}

/// Joins `words` with `delimiter` and applies casing.
///
/// If `lowercase` is `true`, the joined string is lowercased; otherwise it is
/// uppercased. Because words from `generate_name` are already lowercase, the
/// lowercase path is a no-op in practice, but remains explicit for correctness.
pub fn format_name(words: &[String], lowercase: bool, delimiter: &str) -> String {
    let joined = words.join(delimiter);
    if lowercase {
        joined.to_lowercase()
    } else {
        joined.to_uppercase()
    }
}

fn main() {
    let cli = Cli::parse();
    let mut rng = rand::rng();
    let words = generate_name(&mut rng);
    let delimiter = cli.delimiter.as_deref().unwrap_or("");
    println!("{}", format_name(&words, cli.lower, delimiter));
}

#[cfg(test)]
mod tests {
    use super::{ADJECTIVES, NOUNS, format_name, generate_name};
    use rand::{SeedableRng, rngs::SmallRng};
    #[test]
    fn test_generate_name_format() {
        let mut rng = SmallRng::seed_from_u64(42);
        let words = generate_name(&mut rng);
        assert_eq!(words.len(), 2);
        let adj = &words[0];
        let noun = &words[1];
        assert!(!adj.is_empty() && adj.chars().all(|c| c.is_ascii_lowercase()));
        assert!(!noun.is_empty() && noun.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_generate_name_from_word_lists() {
        let mut rng = SmallRng::seed_from_u64(42);
        let words = generate_name(&mut rng);
        assert_eq!(words.len(), 2);
        assert!(ADJECTIVES.contains(&words[0].as_str()) && NOUNS.contains(&words[1].as_str()));
    }

    #[test]
    fn test_generate_name_deterministic() {
        let a = generate_name(&mut SmallRng::seed_from_u64(42));
        let b = generate_name(&mut SmallRng::seed_from_u64(42));
        assert_eq!(a, b);
    }

    fn words(a: &str, b: &str) -> Vec<String> {
        vec![a.to_string(), b.to_string()]
    }

    #[test]
    fn format_name_uppercase_no_delimiter_produces_all_caps_concatenated() {
        assert_eq!(
            format_name(&words("bold", "falcon"), false, ""),
            "BOLDFALCON"
        );
    }

    #[test]
    fn format_name_lowercase_no_delimiter_produces_lowercase_concatenated() {
        assert_eq!(
            format_name(&words("bold", "falcon"), true, ""),
            "boldfalcon"
        );
    }

    #[test]
    fn format_name_uppercase_with_hyphen_delimiter_produces_all_caps_hyphenated() {
        assert_eq!(
            format_name(&words("bold", "falcon"), false, "-"),
            "BOLD-FALCON"
        );
    }

    #[test]
    fn format_name_lowercase_with_hyphen_delimiter_produces_lowercase_hyphenated() {
        assert_eq!(
            format_name(&words("bold", "falcon"), true, "-"),
            "bold-falcon"
        );
    }

    #[test]
    fn format_name_uppercase_with_underscore_delimiter_produces_all_caps_underscored() {
        assert_eq!(
            format_name(&words("bold", "falcon"), false, "_"),
            "BOLD_FALCON"
        );
    }

    #[test]
    fn format_name_empty_delimiter_produces_same_result_as_no_delimiter() {
        let with_empty = format_name(&words("bold", "falcon"), false, "");
        let with_none = format_name(&words("bold", "falcon"), false, "");
        assert_eq!(with_empty, with_none);
        assert_eq!(with_empty, "BOLDFALCON");
    }
}
