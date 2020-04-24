#![feature(try_blocks)]
#![feature(min_specialization)]

use error_return_trace::MyResult;
use error_return_trace::Track;
use std::panic::Location;

#[derive(Debug, Default)]
pub struct ReturnTrace {
    frames: Vec<&'static Location<'static>>,
}

impl Track for ReturnTrace {
    fn track(&mut self, location: &'static Location<'static>) {
        self.frames.push(location);
    }
}

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
