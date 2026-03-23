use rand::Rng;

const ADJECTIVES: &[&str] = &[
    "bold", "bright", "calm", "eager", "fierce", "gentle", "happy", "keen", "lively", "noble",
];

const NOUNS: &[&str] = &[
    "cloud", "falcon", "grove", "harbor", "island", "jaguar", "koala", "lantern", "meadow", "river",
];

/// Picks one random adjective and one random noun and returns them as `"adjective-noun"`.
pub fn generate_name(rng: &mut impl Rng) -> String {
    let adjective = ADJECTIVES[rng.random_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rng.random_range(0..NOUNS.len())];
    format!("{adjective}-{noun}")
}

fn main() {
    let mut rng = rand::rng();
    println!("{}", generate_name(&mut rng));
}

#[cfg(test)]
mod tests {
    use super::{ADJECTIVES, NOUNS, generate_name};
    use rand::{SeedableRng, rngs::SmallRng};
    #[test]
    fn test_generate_name_format() {
        let mut rng = SmallRng::seed_from_u64(42);
        let name = generate_name(&mut rng);
        let (adj, noun) = name.split_once('-').expect("must contain hyphen");
        assert!(!adj.is_empty() && adj.chars().all(|c| c.is_ascii_lowercase()));
        assert!(!noun.is_empty() && noun.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_generate_name_from_word_lists() {
        let mut rng = SmallRng::seed_from_u64(42);
        let name = generate_name(&mut rng);
        let (adj, noun) = name.split_once('-').expect("output must contain a hyphen");
        assert!(ADJECTIVES.contains(&adj) && NOUNS.contains(&noun));
    }

    #[test]
    fn test_generate_name_deterministic() {
        let a = generate_name(&mut SmallRng::seed_from_u64(42));
        let b = generate_name(&mut SmallRng::seed_from_u64(42));
        assert_eq!(a, b);
    }
}
