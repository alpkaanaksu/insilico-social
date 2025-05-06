use rand::Rng;

pub fn random_bool_with_prob(prob: f64) -> bool {
    rand::rng().random_bool(prob)
}

pub fn random_float_in_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..=max)
}

pub fn random_int_in_range(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..=max)
}

pub fn random_prob() -> f64 {
    rand::rng().random_range(0.0..=1.0)
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
