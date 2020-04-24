#![feature(try_blocks)]
use error_return_trace::{MyResult, ReturnTrace};

fn main() {
    let trace = match one() {
        MyResult::Ok(()) => unreachable!(),
        MyResult::Err(trace) => trace,
    };

    println!("{:?}", trace);
}

fn one() -> MyResult<(), ReturnTrace> {
    try { two()? }
}

fn two() -> MyResult<(), ReturnTrace> {
    MyResult::Err(ReturnTrace::default())?
}
