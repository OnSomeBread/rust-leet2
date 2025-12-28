#![allow(unused)]
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tracing::{error, info};

#[macro_export]
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

#[macro_export]
macro_rules! vecstrs {
    ($($x:expr),* $(,)?) => (
        vec![$($x.to_string()),*]
    );

    ($elem:expr; $n:expr) => (
        vec![$elem.to_string(); $n]
    );
}

const TESTS: &str = include_str!("../test/tests.txt");
const ANSWERS: &str = include_str!("../test/ans.txt");

pub trait Test<Args> {
    fn test_print(&self);
}

impl<T1, A, F> Test<(T1, A)> for F
where
    T1: DeserializeOwned,
    A: Debug,
    F: Fn(T1) -> A,
{
    fn test_print(&self) {
        for arg1 in TESTS.lines() {
            println!(
                "ans = {:?}",
                self.call((serde_json::from_str(arg1).unwrap(),))
            );
        }
    }
}

impl<T1, T2, A, F> Test<(T1, T2, A)> for F
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    A: Debug,
    F: Fn(T1, T2) -> A,
{
    fn test_print(&self) {
        let mut lines = TESTS.lines();

        while let (Some(arg1), Some(arg2)) = (lines.next(), lines.next()) {
            let a1 = serde_json::from_str(arg1).unwrap();
            let a2 = serde_json::from_str(arg2).unwrap();
            println!("ans = {:?}", self.call((a1, a2)));
        }
    }
}

impl<T1, T2, T3, A, F> Test<(T1, T2, T3, A)> for F
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    A: Debug,
    F: Fn(T1, T2, T3) -> A,
{
    fn test_print(&self) {
        let mut lines = TESTS.lines();

        while let (Some(arg1), Some(arg2), Some(arg3)) = (lines.next(), lines.next(), lines.next())
        {
            let a1 = serde_json::from_str(arg1).unwrap();
            let a2 = serde_json::from_str(arg2).unwrap();
            let a3 = serde_json::from_str(arg3).unwrap();
            println!("ans = {:?}", self.call((a1, a2, a3)));
        }
    }
}

impl<T1, T2, T3, T4, A, F> Test<(T1, T2, T3, T4, A)> for F
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    T4: DeserializeOwned,
    A: Debug,
    F: Fn(T1, T2, T3, T4) -> A,
{
    fn test_print(&self) {
        let mut lines = TESTS.lines();

        while let (Some(arg1), Some(arg2), Some(arg3), Some(arg4)) =
            (lines.next(), lines.next(), lines.next(), lines.next())
        {
            let a1 = serde_json::from_str(arg1).unwrap();
            let a2 = serde_json::from_str(arg2).unwrap();
            let a3 = serde_json::from_str(arg3).unwrap();
            let a4 = serde_json::from_str(arg4).unwrap();
            println!("ans = {:?}", self.call((a1, a2, a3, a4)));
        }
    }
}

pub trait TestAnswers<Args> {
    fn parse_tests(args_count: u8) -> Vec<(Vec<&'static str>, &'static str)> {
        let mut tests = TESTS.lines();
        let mut answers = ANSWERS.lines();

        let mut res = vec![];
        while let (Some(line), Some(ans)) = (tests.next(), answers.next()) {
            let mut args = vec![line];
            for _ in 1..args_count {
                if let Some(line) = tests.next() {
                    args.push(line);
                }
            }

            res.push((args, ans));
        }

        res
    }
    fn test(&self);
}

impl<F, T1, A> TestAnswers<(T1, A)> for F
where
    T1: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
    F: Fn(T1) -> A + std::marker::Sync,
{
    fn test(&self) {
        Self::parse_tests(1).par_iter().for_each(|(a, ans)| {
            let v1 = self.call((serde_json::from_str::<T1>(a[0]).unwrap(),));
            let v2 = serde_json::from_str(ans).unwrap();
            if v1 == v2 {
                info!("PASSED");
            } else {
                error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
            }
        });
    }
}

impl<F, T1, T2, A> TestAnswers<(T1, T2, A)> for F
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
    F: Fn(T1, T2) -> A + std::marker::Sync,
{
    fn test(&self) {
        Self::parse_tests(2).par_iter().for_each(|(a, ans)| {
            let v1 = self.call((
                serde_json::from_str(a[0]).unwrap(),
                serde_json::from_str(a[1]).unwrap(),
            ));
            let v2 = serde_json::from_str(ans).unwrap();
            if v1 == v2 {
                info!("PASSED");
            } else {
                error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
            }
        });
    }
}

impl<F, T1, T2, T3, A> TestAnswers<(T1, T2, T3, A)> for F
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
    F: Fn(T1, T2, T3) -> A + std::marker::Sync,
{
    fn test(&self) {
        Self::parse_tests(3).par_iter().for_each(|(a, ans)| {
            let v1 = self.call((
                serde_json::from_str(a[0]).unwrap(),
                serde_json::from_str(a[1]).unwrap(),
                serde_json::from_str(a[2]).unwrap(),
            ));
            let v2 = serde_json::from_str(ans).unwrap();
            if v1 == v2 {
                info!("PASSED");
            } else {
                error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
            }
        });
    }
}

impl<F, T1, T2, T3, T4, A> TestAnswers<(T1, T2, T3, T4, A)> for F
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    T4: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
    F: Fn(T1, T2, T3, T4) -> A + std::marker::Sync,
{
    fn test(&self) {
        Self::parse_tests(4).par_iter().for_each(|(a, ans)| {
            let v1 = self.call((
                serde_json::from_str(a[0]).unwrap(),
                serde_json::from_str(a[1]).unwrap(),
                serde_json::from_str(a[2]).unwrap(),
                serde_json::from_str(a[3]).unwrap(),
            ));
            let v2 = serde_json::from_str(ans).unwrap();
            if v1 == v2 {
                info!("PASSED");
            } else {
                error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
            }
        });
    }
}

pub fn t<F, Args>(f: F)
where
    F: Test<Args>,
{
    f.test_print();
}

pub fn ta<F, Args>(f: F)
where
    F: TestAnswers<Args>,
{
    f.test();
}
