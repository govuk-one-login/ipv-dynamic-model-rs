use rand::{random, random_range};

#[must_use]
pub fn random_string(prefix: &str) -> String {
    format!("{prefix} {:08}", random_range(10_000_000..=99_999_999))
}

#[must_use]
pub fn random_vec<T, F: Fn() -> T>(min: usize, max: usize, generator: F) -> Vec<T> {
    (0..random_range(min..max)).map(|_| generator()).collect()
}

pub trait RandomChoice: Sized {
    fn random_choice() -> Self;

    #[must_use]
    fn random_choice_option(chance_of_option: f64) -> Option<Self> {
        let random_number = random::<f64>();
        (random_number < chance_of_option).then(Self::random_choice)
    }
}

pub trait CreateTestSubject: Sized {
    fn create_test_subject() -> Self;
}

#[must_use]
pub fn approximately_eq_f64(left: f64, right: f64) -> bool {
    (left - right).abs() < 0.00001
}
