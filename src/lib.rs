#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#[macro_export]
macro_rules! include_data {
    ($day:expr) => {{
        // need the type spec or rust_analyzer is not happy with me
        // https://users.rust-lang.org/t/macro-return-type/58596/6
        let out: &str = include_str!(concat!("../", stringify!($day), "/input.txt"));
        out
    }};
    ($day:expr, sample) => {{
        let out: &str = include_str!(concat!("../", stringify!($day), "/sample.txt"));
        out
    }};
}

pub mod math {
    #[must_use]
    pub fn lcm(first: u64, second: u64) -> u64 {
        (first * second) / gcd(first, second)
    }

    fn gcd(first: u64, second: u64) -> u64 {
        let mut max = first;
        let mut min = second;
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }
            max = min;
            min = res;
        }
    }
}
