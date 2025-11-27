#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_range_loop)]

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use rayon::prelude::*;
use tracing::info;

mod first_4k_lines;

#[allow(unused)]
macro_rules! vecvec {
    () => {
        Vec::new()
    };
    ( $( [ $( $x:expr ),* ] ),* $(,)? ) => {
        vec![ $( vec![ $( $x ),* ] ),* ]
    };
    ( $( [ $elem:expr; $n:expr ] ),* $(,)? ) => {
        vec![ $( vec![$elem; $n] ),* ]
    };
}

#[allow(unused)]
macro_rules! vecstrs {
    ($($x:expr),* $(,)?) => (
        vec![$($x.to_string()),*]
    );

    ($elem:expr; $n:expr) => (
        vec![$elem.to_string(); $n]
    );
}

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut remainder = vec![i32::MAX; k as usize];
    5
}

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .without_time()
        .init();

    info!("{:?}", first_4k_lines::max_subarray_sum(vec![1, 2], 1));
}
