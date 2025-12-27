#![allow(unused)]
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

pub fn t1<T, A>(f: fn(T) -> A)
where
    T: DeserializeOwned,
    A: Debug,
{
    for arg1 in TESTS.lines() {
        info!("{:?}", f(serde_json::from_str(arg1).unwrap()));
    }
}

pub fn t2<T1, T2, A>(f: fn(T1, T2) -> A)
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    A: Debug,
{
    let mut lines = TESTS.lines();

    while let (Some(arg1), Some(arg2)) = (lines.next(), lines.next()) {
        let a1 = serde_json::from_str(arg1).unwrap();
        let a2 = serde_json::from_str(arg2).unwrap();
        info!("{:?}", f(a1, a2));
    }
}

pub fn t3<T1, T2, T3, A>(f: fn(T1, T2, T3) -> A)
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    A: Debug,
{
    let mut lines = TESTS.lines();

    while let (Some(arg1), Some(arg2), Some(arg3)) = (lines.next(), lines.next(), lines.next()) {
        let a1 = serde_json::from_str(arg1).unwrap();
        let a2 = serde_json::from_str(arg2).unwrap();
        let a3 = serde_json::from_str(arg3).unwrap();
        info!("{:?}", f(a1, a2, a3));
    }
}

pub fn t4<T1, T2, T3, T4, A>(f: fn(T1, T2, T3, T4) -> A)
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    T4: DeserializeOwned,
    A: Debug,
{
    let mut lines = TESTS.lines();

    while let (Some(arg1), Some(arg2), Some(arg3), Some(arg4)) =
        (lines.next(), lines.next(), lines.next(), lines.next())
    {
        let a1 = serde_json::from_str(arg1).unwrap();
        let a2 = serde_json::from_str(arg2).unwrap();
        let a3 = serde_json::from_str(arg3).unwrap();
        let a4 = serde_json::from_str(arg4).unwrap();
        info!("{:?}", f(a1, a2, a3, a4));
    }
}

pub fn t1a<T, A>(f: fn(T) -> A)
where
    T: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
{
    for (test, ans) in TESTS.lines().zip(ANSWERS.lines()) {
        let v1 = f(serde_json::from_str(test).unwrap());
        let v2 = serde_json::from_str(ans).unwrap();
        if v1 == v2 {
            info!("PASSED");
        } else {
            error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
        }
    }
}

pub fn t2a<T1, T2, A>(f: fn(T1, T2) -> A)
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
{
    let mut tests = TESTS.lines();
    let mut answers = ANSWERS.lines();

    while let (Some(a1), Some(a2), Some(ans)) = (tests.next(), tests.next(), answers.next()) {
        let v1 = f(
            serde_json::from_str(a1).unwrap(),
            serde_json::from_str(a2).unwrap(),
        );
        let v2 = serde_json::from_str(ans).unwrap();
        if v1 == v2 {
            info!("PASSED");
        } else {
            error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
        }
    }
}

pub fn t3a<T1, T2, T3, A>(f: fn(T1, T2, T3) -> A)
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
{
    let mut tests = TESTS.lines();
    let mut answers = ANSWERS.lines();

    while let (Some(a1), Some(a2), Some(a3), Some(ans)) =
        (tests.next(), tests.next(), tests.next(), answers.next())
    {
        let v1 = f(
            serde_json::from_str(a1).unwrap(),
            serde_json::from_str(a2).unwrap(),
            serde_json::from_str(a3).unwrap(),
        );
        let v2 = serde_json::from_str(ans).unwrap();
        if v1 == v2 {
            info!("PASSED");
        } else {
            error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
        }
    }
}

pub fn t4a<T1, T2, T3, T4, A>(f: fn(T1, T2, T3, T4) -> A)
where
    T1: DeserializeOwned,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    T4: DeserializeOwned,
    A: DeserializeOwned + Debug + PartialEq,
{
    let mut tests = TESTS.lines();
    let mut answers = ANSWERS.lines();

    while let (Some(a1), Some(a2), Some(a3), Some(a4), Some(ans)) = (
        tests.next(),
        tests.next(),
        tests.next(),
        tests.next(),
        answers.next(),
    ) {
        let v1 = f(
            serde_json::from_str(a1).unwrap(),
            serde_json::from_str(a2).unwrap(),
            serde_json::from_str(a3).unwrap(),
            serde_json::from_str(a4).unwrap(),
        );
        let v2 = serde_json::from_str(ans).unwrap();
        if v1 == v2 {
            info!("PASSED");
        } else {
            error!("FAILED FOUND: {:?} EXPECTED: {:?}", v1, v2);
        }
    }
}
