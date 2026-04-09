use rand::{RngExt, distr::Alphanumeric};

pub fn generate_suffix() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>()
        .to_lowercase()
}
