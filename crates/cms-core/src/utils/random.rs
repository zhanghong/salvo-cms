use rand::Rng;
use rand::distr::Alphanumeric;

pub fn alpha_string(length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}
