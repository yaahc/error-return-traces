#![feature(try_blocks)]
#![feature(min_specialization)]

use error_return_trace::MyResult;
use error_return_trace::Track;
use std::fmt;
use std::panic::Location;

#[derive(Default)]
pub struct ReturnTrace {
    frames: Vec<&'static Location<'static>>,
}

impl Track for ReturnTrace {
    fn track(&mut self, location: &'static Location<'static>) {
        self.frames.push(location);
    }
}

impl fmt::Debug for ReturnTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ReturnTrace:")?;
        for (i, frame) in self.frames.iter().enumerate() {
            writeln!(f, "{:>2}: {}", i, frame)?;
        }

        Ok(())
    }
}

fn main() -> MyResult<(), ReturnTrace> {
    try { one()? }
}

fn one() -> MyResult<(), ReturnTrace> {
    try { two()? }
}

fn two() -> MyResult<(), ReturnTrace> {
    MyResult::Err(ReturnTrace::default())?
}
