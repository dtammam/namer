mod adjectives;
mod objects;

pub(crate) use adjectives::ADJECTIVES;
/// Backward-compatible alias for OBJECTS. Will be removed in Task 2.
pub(crate) use objects::NOUNS;
#[allow(unused_imports)]
pub(crate) use objects::OBJECTS;
