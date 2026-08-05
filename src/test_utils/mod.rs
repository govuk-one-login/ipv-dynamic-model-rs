use rand::random_range;

pub fn random_string(prefix: &str) -> String {
    format!("{prefix} {:08}", rand::random_range(10000000..=99999999))
}

pub fn random_vec<T, F: Fn() -> T>(min: usize, max: usize, generator: F) -> Vec<T> {
    (0..random_range(min..max)).map(|_| generator()).collect()
}

pub trait RandomChoice: Sized {
    fn random_choice() -> Self;

    fn random_choice_option(chance_of_option: f64) -> Option<Self> {
        let random_number = rand::random::<f64>();
        (random_number < chance_of_option).then(Self::random_choice)
    }
}

pub trait CreateTestSubject: Sized {
    fn create_test_subject() -> Self;
}
